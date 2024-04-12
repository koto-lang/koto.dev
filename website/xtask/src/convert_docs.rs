use anyhow::{bail, Context, Result};
use pulldown_cmark::{CodeBlockKind, CowStr, Event, HeadingLevel, Parser, Tag};
use pulldown_cmark_to_cmark::cmark;
use std::{
    fs,
    iter::once,
    ops::Deref,
    path::{Path, PathBuf},
};

pub fn run() -> Result<()> {
    convert_single_page_doc(
        "about.md",
        "about",
        r#"+++
title = "About"
template = "docs-guide.html"
insert_anchor_links = "heading"
weight = 1
+++
"#,
        true,
        false,
    )?;
    convert_single_page_doc(
        "language_guide.md",
        "language",
        r#"+++
title = "Language Guide"
template = "docs-guide.html"
insert_anchor_links = "heading"
weight = 2
+++

# The Koto Language Guide

As you're reading this guide, you're encouraged to play around with the examples to get a feel
for the language.

When you see a <span uk-icon="play"></span> icon below an example,
clicking it will open the example in the [Koto Playground](https://koto.dev/play),
where you can run the code and see what happens as you make changes.
"#,
        true,
        true,
    )?;
    convert_doc_folder("../modules/koto/docs/core_lib", "content/docs/next/core", true)?;
    convert_doc_folder("../modules/koto/docs/libs", "content/docs/next/libs", false)?;
    convert_single_page_doc(
        "cli.md",
        "cli",
        r#"+++
title = "Koto CLI"
template = "docs-guide.html"
insert_anchor_links = "heading"
weight = 5
+++
"#,
        false,
        false,
    )?;
    convert_single_page_doc(
        "api.md",
        "api",
        r#"+++
title = "Rust API"
template = "docs-guide.html"
insert_anchor_links = "heading"
weight = 6
+++
"#,
        true,
        false,
    )?;

    println!("Docs updated");

    Ok(())
}

fn convert_single_page_doc(
    input_file: &str,
    output_sub_dir: &str,
    intro: &str,
    skip_preamble: bool,
    skip_title: bool,
) -> Result<()> {
    use std::io::Write;

    let mut input_path = PathBuf::from("../modules/koto/docs/");
    input_path.push(input_file);
    let mut output_dir = PathBuf::from("content/docs/next/");
    output_dir.push(output_sub_dir);

    let mut output_path = output_dir.clone();
    output_path.push("_index.md");
    let mut output_file = fs::File::create(&output_path)
        .with_context(|| format!("Failed to create output file '{output_path:?}'"))?;
    write!(output_file, "{intro}")?;

    let converted = convert_doc(&input_path, false, skip_preamble, skip_title, true)?;
    write!(output_file, "\n\n{converted}")?;

    Ok(())
}

fn convert_doc_folder(input: &str, output: &str, add_playground_links: bool) -> Result<()> {
    use std::io::Write;

    let output_dir = PathBuf::from(output);

    for doc in fs::read_dir(input)? {
        let doc_path = doc?.path();
        let converted = convert_doc(&doc_path, true, false, false, add_playground_links)?;

        let mut output_path = output_dir.clone();
        output_path.push(doc_path.file_name().unwrap());
        let mut output_file = fs::File::create(&output_path)
            .with_context(|| format!("Failed to create output file '{output_path:?}'"))?;

        write!(output_file, "{converted}")?;
    }

    Ok(())
}

fn skip_until<'a>(input: &'a str, token: &str) -> Result<&'a str> {
    let Some((_, skipped)) = input.split_once(token) else {
        bail!("Couldn't find token '{token}'");
    };
    let Some((_, skipped)) = skipped.split_once('\n') else {
        bail!("Couldn't find newline after token '{token}'");
    };

    Ok(skipped)
}

fn convert_doc(
    input_path: &Path,
    generate_front_matter: bool,
    skip_preamble: bool,
    skip_title: bool,
    add_playgound_links: bool,
) -> Result<String> {
    use {std::fmt::Write, Event::*, Tag::*};

    let input_contents = fs::read_to_string(input_path)?;

    let input = if skip_preamble {
        skip_until(&input_contents, "---")?
    } else {
        &input_contents
    };

    let input = if skip_title {
        skip_until(input, "# ")?
    } else {
        input
    };
    // Write out the modified markdown with Zola front matter
    let mut output_buffer = String::with_capacity(input.len());

    if generate_front_matter {
        let slug = input_path
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        let entry_name = {
            let mut in_heading = false;
            let mut entry_name = None;
            for event in Parser::new(&input) {
                match event {
                    Start(Heading(HeadingLevel::H1, _, _)) => {
                        in_heading = true;
                    }
                    Text(text) if in_heading => {
                        entry_name = Some(text.to_string());
                        break;
                    }
                    _ => {}
                }
            }

            entry_name.unwrap_or_else(|| slug.clone())
        };

        write!(
            output_buffer,
            "\
+++
title = \"{entry_name}\"
slug = \"{slug}\"
",
        )?;

        writeln!(output_buffer, "+++\n")?;
    }

    // Parse the input markdown and perform some modifications
    // Each event is converted into an iterator providing modified events,
    // with flat_map merging the iterators back into a single event stream.
    let parser = Parser::new(&input).flat_map({
        // Add a playground link to every koto code block
        let mut in_koto_code = false;
        let mut in_rust_include = false;
        let mut koto_code = CowStr::from("");
        move |event| match event {
            Start(CodeBlock(CodeBlockKind::Fenced(ref lang))) => {
                match lang.split(',').next() {
                    Some("koto") => {
                        in_koto_code = true;
                        // Split off the language modifier to avoid confusing zola
                        once(Start(CodeBlock(CodeBlockKind::Fenced("koto".into())))).chain(None)
                    }
                    Some("rust_include") => {
                        in_rust_include = true;
                        once(Start(CodeBlock(CodeBlockKind::Fenced("rust".into())))).chain(None)
                    }
                    _ => once(event).chain(None),
                }
            }
            End(CodeBlock(CodeBlockKind::Fenced(_))) if in_koto_code => {
                in_koto_code = false;
                if add_playgound_links {
                    let playground_code = koto_code
                        .deref()
                        .replace("print! ", "print ")
                        .replace("check! ", "# -> ")
                        .replace("skip_check!\n", "")
                        .replace("skip_run!\n", "");
                    let shortcode = format!(
                        "\
{{% example_playground_link() %}}
{playground_code}
{{% end %}}
"
                    );
                    once(event).chain(Some(Text(shortcode.into())))
                } else {
                    once(event).chain(None)
                }
            }
            End(CodeBlock(CodeBlockKind::Fenced(_))) if in_rust_include => {
                in_rust_include = false;
                once(event).chain(None)
            }
            End(Link(link_type, url, title)) => {
                once(End(Link(link_type, fix_url(&url).into(), title))).chain(None)
            }
            End(FootnoteDefinition(url)) => {
                once(End(FootnoteDefinition(fix_url(&url).into()))).chain(None)
            }
            Text(code) if in_koto_code => {
                koto_code = code.clone();
                let display_code = koto_code
                    .deref()
                    .replace("print! ", "")
                    .replace("check! ", "# -> ");
                once(Text(display_code.into())).chain(None)
            }
            Text(file_name) if in_rust_include => {
                let file_name = file_name.trim();
                let path = format!("../modules/koto/crates/koto/examples/{file_name}");
                let rust_file_contents = fs::read_to_string(&path)
                    .map_err(|e| format!("Failed to include rust file from '{path}' ({e})"))
                    .unwrap();
                once(Text(rust_file_contents.into())).chain(None)
            }
            _ => once(event).chain(None),
        }
    });

    cmark(parser, &mut output_buffer)?;

    Ok(output_buffer)
}

fn fix_url(url: &str) -> String {
    url
        // Fix guide->core_lib links
        .replace("../core_lib", "../core")
        // Fix core_lib->guide links
        .replace("../language_guide.md", "../../language/")
        // Fix top level docs -> guide links
        .replace("./language_guide.md", "../language/")
}
