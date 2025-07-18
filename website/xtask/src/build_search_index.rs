use std::{
    collections::HashSet, ffi::OsStr, fs, iter::Peekable, mem, os::unix::fs::MetadataExt,
    path::Path,
};

use anyhow::{bail, Context, Result};
use pulldown_cmark::{HeadingLevel, Parser};
use serde::Serialize;
use sluggify::sluggify::sluggify;

pub fn run() -> Result<()> {
    let index = IndexBuilder::build_index()?;

    let index_json =
        serde_json::to_string(&index.entries).context("failed to serialize the index as JSON")?;

    let output_path = "static/search-index-next.json";
    fs::write(output_path, index_json)?;
    let output_size = (fs::metadata(output_path)?.size() as f64 / 1000.0).round();

    println!(
        "Search index built ({} entries, {output_size}kB)",
        index.entries.len(),
    );

    Ok(())
}

struct IndexBuilder {
    entries: Vec<SearchEntry>,
}

impl IndexBuilder {
    fn build_index() -> Result<Self> {
        let mut builder = Self {
            entries: Vec::default(),
        };

        builder.add_file(
            "../modules/koto/docs/language_guide.md",
            "/docs/next/language/",
            "Language Guide",
        )?;
        builder.add_file("../modules/koto/docs/api.md", "/docs/next/api/", "API")?;
        builder.add_file("../modules/koto/docs/cli.md", "/docs/next/cli/", "CLI")?;
        builder.add_lib_dir(
            "../modules/koto/docs/core_lib/",
            "/docs/next/core/",
            "Core Library",
        )?;
        builder.add_lib_dir(
            "../modules/koto/docs/libs/",
            "/docs/next/libs/",
            "Extra Libs",
        )?;

        Ok(builder)
    }

    fn add_entry(&mut self, entry: SearchEntry) {
        if !entry.contents.is_empty() {
            self.entries.push(entry);
        }
    }

    fn add_file(&mut self, path: &str, base_url: &str, module: &str) -> Result<()> {
        let contents =
            fs::read_to_string(path).with_context(|| format!("failed to read '{path}'"))?;

        let mut parser = Parser::new(&contents).peekable();

        // Add all non-empty sections as separate search entries
        while let Some(entry) =
            SearchEntry::parse_section(&mut parser, base_url, None, module, None, false)?
        {
            self.add_entry(entry);

            if parser.peek().is_none() {
                break;
            }
        }

        Ok(())
    }

    fn add_lib_dir(&mut self, dir: &str, base_url: &str, module: &str) -> Result<()> {
        for entry in fs::read_dir(dir).with_context(|| format!("failed to read '{dir}'"))? {
            let entry = entry?;

            let path = entry.path();
            if matches!(path.extension().and_then(OsStr::to_str), Some("md")) {
                self.add_lib_file(&path, base_url, module)?;
            }
        }

        Ok(())
    }

    fn add_lib_file(&mut self, path: &Path, dir_url: &str, section_name: &str) -> Result<()> {
        let contents =
            fs::read_to_string(path).with_context(|| format!("failed to read '{path:?}'"))?;

        let mut parser = Parser::new(&contents).peekable();

        let Some(module_name) = path
            .file_stem()
            .and_then(OsStr::to_str)
            .map(|name| name.to_string())
        else {
            bail!("Missing file name for '{path:?}'");
        };

        let base_url = format!("{dir_url}{module_name}/");

        let Some(intro) = SearchEntry::parse_section(
            &mut parser,
            &base_url,
            None,
            section_name,
            Some(HeadingLevel::H1),
            false,
        )?
        else {
            bail!("Missing intro section in '{path:?}'");
        };
        self.add_entry(intro);

        // Add all H2 sections as separate search entries, skipping H3+ subsections
        while let Some(entry) = SearchEntry::parse_section(
            &mut parser,
            &base_url,
            Some(&module_name),
            section_name,
            None,
            true,
        )? {
            self.add_entry(entry);

            if parser.peek().is_none() {
                break;
            }
        }

        Ok(())
    }
}

#[derive(Default, Serialize)]
struct SearchEntry {
    title: String,
    module: String,
    url: String,
    contents: String,
    keywords: HashSet<String>,
}

impl SearchEntry {
    fn parse_section(
        parser: &mut Peekable<Parser>,
        base_url: &str,
        title_prefix: Option<&str>,
        module: &str,
        mut level_to_consume: Option<HeadingLevel>,
        skip_sub_sections: bool,
    ) -> Result<Option<SearchEntry>> {
        use pulldown_cmark::{CodeBlockKind, Event::*, Tag, TagEnd};
        use std::cmp::Ordering::*;

        let mut contents = String::new();
        let mut section_name = String::new();
        let mut keywords = HashSet::new();
        let mut keyword = String::new();

        #[derive(Debug)]
        enum ParsingMode {
            WaitingForSectionStart,
            Any,
            Section,
            SubSection,
            Code,
            TypeDeclaration,
        }

        let mut capture_keyword = false;

        let mut parsing_mode = ParsingMode::WaitingForSectionStart;

        while let Some(peeked) = parser.peek() {
            let waiting_for_start = matches!(parsing_mode, ParsingMode::WaitingForSectionStart);
            if waiting_for_start {
                match peeked {
                    Start(Tag::Heading { level, .. }) => {
                        if let Some(level_to_consume) = level_to_consume {
                            match level.cmp(&level_to_consume) {
                                Equal => parsing_mode = ParsingMode::Section,
                                Greater => {
                                    // Continue until the start of the requested section is found
                                }
                                Less => break,
                            }
                        } else {
                            parsing_mode = ParsingMode::Section;
                            level_to_consume = Some(*level);
                        }
                    }
                    _ => {}
                }
            } else {
                match peeked {
                    Start(Tag::Heading { level, .. }) => {
                        if let Some(level_to_consume) = level_to_consume {
                            match level.cmp(&level_to_consume) {
                                Greater if skip_sub_sections => {
                                    parsing_mode = ParsingMode::SubSection
                                }
                                _ => break,
                            }
                        } else {
                            unreachable!();
                        }
                    }
                    End(TagEnd::Heading(_)) => parsing_mode = ParsingMode::Any,
                    Start(Tag::Link { title, .. }) => contents.push_str(title),
                    End(TagEnd::Link) => {}
                    End(TagEnd::Item) => {}
                    Start(Tag::Paragraph) => {
                        if !contents.is_empty() {
                            contents.push_str(" ")
                        }
                    }
                    End(TagEnd::Paragraph) => {}
                    Start(Tag::CodeBlock(CodeBlockKind::Fenced(lang))) => {
                        match lang.split(',').next() {
                            Some("kototype") => parsing_mode = ParsingMode::TypeDeclaration,
                            _ => parsing_mode = ParsingMode::Code,
                        }
                    }
                    End(TagEnd::CodeBlock) => parsing_mode = ParsingMode::Any,
                    Start(Tag::Emphasis) => capture_keyword = true,
                    End(TagEnd::Emphasis) => {
                        if !keyword.is_empty() {
                            keywords.insert(mem::take(&mut keyword));
                            capture_keyword = false;
                        }
                    }
                    Text(text) => match parsing_mode {
                        ParsingMode::WaitingForSectionStart => {}
                        ParsingMode::Any => {
                            contents.push_str(text);
                            if capture_keyword {
                                keyword.push_str(text);
                            }
                        }
                        ParsingMode::Section => section_name.push_str(text),
                        ParsingMode::SubSection => {
                            debug_assert!(skip_sub_sections);
                        }
                        ParsingMode::Code => {
                            // Skipping code
                        }
                        ParsingMode::TypeDeclaration => {
                            // Skipping type declarations
                        }
                    },
                    Code(code) => match parsing_mode {
                        ParsingMode::Section => {
                            section_name.push_str(code);
                        }
                        ParsingMode::SubSection => {
                            debug_assert!(skip_sub_sections);
                        }
                        ParsingMode::Any => {
                            contents.push_str(code);
                        }
                        _ => {}
                    },
                    SoftBreak => contents.push(' '),
                    HardBreak => contents.push('\n'),
                    _other => {}
                }
            }

            parser.next();
        }

        let url = format!("{base_url}#{}", sluggify(&section_name, None));
        let title = if let Some(prefix) = title_prefix {
            format!("{prefix}.{section_name}")
        } else {
            section_name
        };

        Ok(Some(SearchEntry {
            title,
            module: module.into(),
            url,
            contents,
            keywords,
        }))
    }
}
