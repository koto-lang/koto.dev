use {
    pulldown_cmark::{Event, Parser, Tag},
    pulldown_cmark_to_cmark::cmark,
    std::{error::Error, fs, io::Write, ops::Deref, path::PathBuf},
};

fn main() -> Result<(), Box<dyn Error>> {
    let output_dir = PathBuf::from("content/docs/core/");

    for entry in fs::read_dir("modules/koto/docs/reference/core_lib")? {
        let entry = entry?;
        let input_path = entry.path();
        let input_contents = fs::read_to_string(&input_path)?;

        use {Event::*, Tag::*};
        let mut in_heading = false;
        let mut reference_found = false;
        let mut allow_events_through = false;
        let parser = Parser::new(&input_contents).filter(|event| {
            // Wait until we reach the '# Reference' heading to start letting events through
            match event {
                Start(Heading(_, _, _)) => {
                    in_heading = true;
                }
                End(Heading(_, _, _)) => {
                    in_heading = false;
                    if !allow_events_through {
                        if reference_found {
                            allow_events_through = true;
                        } else {
                            return false;
                        }
                    }
                }
                Text(text) if in_heading && text.deref() == "Reference" => {
                    reference_found = true;
                }
                _ => {}
            }
            allow_events_through
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
title = \"{name}\"
+++

# {name}

",
            name = input_path.file_stem().unwrap().to_str().unwrap(),
        )?;

        write!(output_file, "{}", &output_buffer)?;
    }

    Ok(())
}
