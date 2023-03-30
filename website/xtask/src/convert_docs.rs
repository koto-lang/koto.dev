use {
    super::Result,
    pulldown_cmark::{CodeBlockKind, CowStr, Event, HeadingLevel, Parser, Tag},
    pulldown_cmark_to_cmark::cmark,
    std::{
        fs,
        iter::once,
        ops::Deref,
        path::{Path, PathBuf},
    },
};

pub fn run() -> Result<()> {
    convert_lang_guide_docs()?;
    convert_core_lib_docs()?;

    println!("Docs updated");

    Ok(())
}

fn convert_lang_guide_docs() -> Result<()> {
    use {std::io::Write, Event::*, Tag::*};

    let guide_dir = PathBuf::from("../modules/koto/docs/language");
    let output_dir = PathBuf::from("content/docs/next/language");

    // Read through the index and convert each guide doc in order
    let mut index_path = guide_dir.clone();
    index_path.push("_index.md");
    let index_contents = fs::read_to_string(index_path)?;

    let mut output_path = output_dir.clone();
    output_path.push("_index.md");
    let mut output_file = fs::File::create(&output_path).map_err(|e| {
        format!(
            "Failed to create output file '{}': '{}'",
            output_path.to_string_lossy(),
            e
        )
    })?;
    write!(output_file, include_str!("../../templates/guide-intro.md"))?;

    let mut in_list_item = false;
    for event in Parser::new(&index_contents) {
        match event {
            Start(Item) => in_list_item = true,
            End(Item) => in_list_item = false,
            Start(Link(_, url, _)) if in_list_item => {
                let mut doc_path = guide_dir.clone();
                doc_path.push(url.as_ref());
                let converted = convert_doc(&doc_path, false, true)?;

                write!(output_file, "\n\n{converted}")?;
            }
            _ => {}
        }
    }

    Ok(())
}

fn convert_core_lib_docs() -> Result<()> {
    use std::io::Write;

    let output_dir = PathBuf::from("content/docs/next/core");

    for doc in fs::read_dir("../modules/koto/docs/core_lib")? {
        let doc_path = doc?.path();
        let converted = convert_doc(&doc_path, true, false)?;

        let mut output_path = output_dir.clone();
        output_path.push(doc_path.file_name().unwrap());
        let mut output_file = fs::File::create(&output_path).map_err(|e| {
            format!(
                "Failed to create output file '{}': '{}'",
                output_path.to_string_lossy(),
                e
            )
        })?;

        write!(output_file, "{converted}")?;
    }

    Ok(())
}

fn convert_doc(
    input_path: &Path,
    generate_front_matter: bool,
    indent_headers: bool,
) -> Result<String> {
    use {std::fmt::Write, Event::*, Tag::*};

    let input_contents = fs::read_to_string(&input_path)?;

    // Write out the modified markdown with Zola front matter
    let mut output_buffer = String::with_capacity(input_contents.len());

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
            for event in Parser::new(&input_contents) {
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
    let parser = Parser::new(&input_contents).flat_map({
        // Add a playground link to every koto code block
        let mut in_koto_code = false;
        let mut koto_code = CowStr::from("");
        move |event| match event {
            Start(CodeBlock(CodeBlockKind::Fenced(ref lang))) => match lang.split(',').next() {
                Some("koto") => {
                    in_koto_code = true;
                    // Split off the language modifier to avoid confusing zola
                    once(Start(CodeBlock(CodeBlockKind::Fenced("koto".into())))).chain(None)
                }
                _ => {
                    in_koto_code = false;
                    once(event).chain(None)
                }
            },
            End(CodeBlock(CodeBlockKind::Fenced(_))) if in_koto_code => {
                in_koto_code = false;
                let playground_code = koto_code
                    .deref()
                    .replace("print! ", "print ")
                    .replace("check! ", "# -> ")
                    .replace("skip_check!\n", "")
                    .replace("skip_run!\n", "");
                let shortcode = format!(
                    "\
{{% example_playground_link() %}}
play.clear_output()

{playground_code}
{{% end %}}
"
                );
                once(event).chain(Some(Text(shortcode.into())))
            }
            Start(Heading(level, fragment, classes)) if indent_headers => {
                let new_level = HeadingLevel::try_from(level as usize + 1).unwrap_or(level);
                once(Start(Heading(new_level, fragment, classes))).chain(None)
            }
            Text(code) if in_koto_code => {
                koto_code = code.clone();
                let display_code = koto_code
                    .deref()
                    .replace("print! ", "")
                    .replace("check! ", "# -> ");
                once(Text(display_code.into())).chain(None)
            }
            _ => once(event).chain(None),
        }
    });

    cmark(parser, &mut output_buffer)?;

    Ok(output_buffer)
}
