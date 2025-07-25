use std::env::args;

use anyhow::{bail, Result};

mod build_search_index;
mod convert_docs;
mod data;
mod postprocess_playground;
mod version_snapshot;

fn main() {
    if let Err(e) = try_main() {
        eprintln!("{}", e);
        std::process::exit(-1);
    }
}

fn try_main() -> Result<()> {
    match args().nth(1).as_deref() {
        Some("docs") => convert_docs::run(),
        Some("playground") => match args().nth(2).as_ref() {
            Some(staging_dir) => postprocess_playground::run(staging_dir),
            None => bail!("Missing argument: staging dir"),
        },
        Some("search-index") => build_search_index::run(),
        Some("version-snapshot") => match args().nth(2).as_ref() {
            Some(version) => version_snapshot::run(version),
            _ => bail!("Missing argument: version"),
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
docs                Converts Koto's docs for Zola
playground          Postprocesses the Koto playground for integration in the main website
search-index        Builds a search index for the docs in content/docs/next
version-snapshot    Takes a versioned snapshot of the docs and playground
";
