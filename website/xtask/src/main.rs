use anyhow::{bail, Result};
use std::env::args;

mod convert_docs;
mod postprocess_playground;

fn main() {
    if let Err(e) = try_main() {
        eprintln!("{}", e);
        std::process::exit(-1);
    }
}

fn try_main() -> Result<()> {
    match args().nth(1).as_deref() {
        Some("docs") => convert_docs::run(),
        Some("playground") => match args().nth(2).as_deref() {
            Some(staging_dir) => postprocess_playground::run(staging_dir),
            None => bail!("Missing argument: staging dir"),
        },
        Some("help" | "--help") => {
            println!("{HELP}");
            Ok(())
        }
        _ => bail!(HELP),
    }
}

const HELP: &str = "\
Tasks:
docs            converts Koto's docs for Zola
playground      postprocesses the Koto playground for integration in the main website
";
