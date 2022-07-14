use {
    pulldown_cmark::{CodeBlockKind, CowStr, Event, HeadingLevel, Parser, Tag},
    pulldown_cmark_to_cmark::cmark,
    std::{
        error::Error,
        fs,
        io::Write,
        iter::once,
        ops::Deref,
        path::{Path, PathBuf},
    },
};

fn main() -> Result<(), Box<dyn Error>> {
    convert_lang_guide_docs()?;
    convert_core_lib_docs()?;

    println!("Docs updated");

    Ok(())
}

fn convert_lang_guide_docs() -> Result<(), Box<dyn Error>> {
    let guide_dir = PathBuf::from("../modules/koto/docs/language");
    let output_dir = PathBuf::from("content/docs/next/language");

    let mut index_path = guide_dir.clone();
    index_path.push("_index.md");
    let index_contents = fs::read_to_string(index_path)?;

    use {Event::*, Tag::*};

    let mut weight = 0;
    let mut in_list_item = false;
    for event in Parser::new(&index_contents) {
        match event {
            Start(Item) => in_list_item = true,
            End(Item) => in_list_item = false,
            Start(Link(_, url, _)) if in_list_item => {
                let mut doc_path = guide_dir.clone();
                doc_path.push(url.as_ref());
                convert_doc(&doc_path, output_dir.clone(), Some(weight))?;
                weight += 1;
            }
            _ => {}
        }
    }

    Ok(())
}

fn convert_core_lib_docs() -> Result<(), Box<dyn Error>> {
    let output_dir = PathBuf::from("content/docs/next/core");

    for doc in fs::read_dir("../modules/koto/docs/core_lib")? {
        convert_doc(&doc?.path(), output_dir.clone(), None)?;
    }

    Ok(())
}

fn convert_doc(
    input_path: &Path,
    output_dir: PathBuf,
    weight: Option<usize>,
) -> Result<(), Box<dyn Error>> {
    let input_contents = fs::read_to_string(&input_path)?;

    use {Event::*, Tag::*};

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

    // Parse the input markdown and perform some modifications
    let parser = Parser::new(&input_contents)
        .flat_map({
            // Add a playground link to every koto code block
            let mut in_koto_code = false;
            let mut koto_code = CowStr::from("");
            move |event| match &event {
                Start(CodeBlock(CodeBlockKind::Fenced(lang))) => match lang.split(',').next() {
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

    let mut output_buffer = String::with_capacity(input_contents.len());
    cmark(parser, &mut output_buffer)?;

    let mut output_path = output_dir.clone();
    output_path.push(input_path.file_name().unwrap());
    let mut output_file = fs::File::create(&output_path).map_err(|e| {
        format!(
            "Failed to create output file '{}': '{}'",
            output_path.to_string_lossy(),
            e
        )
    })?;

    // Write out the modified markdown with Zola front matter
    write!(
        output_file,
        "\
+++
title = \"{entry_name}\"
slug = \"{slug}\"
",
    )?;

    if let Some(weight) = weight {
        writeln!(output_file, "weight = {weight}")?;
    }

    writeln!(output_file, "+++\n")?;

    write!(output_file, "{}", &output_buffer)?;

    Ok(())
}
