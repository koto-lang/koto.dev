use anyhow::{Context, Result};
use std::{fs, io::Write, path::PathBuf};

pub fn run(staging_dir: &str) -> Result<()> {
    let index_path: PathBuf = [staging_dir, "index.html"].iter().collect();

    let index = fs::read_to_string(&index_path).context("Failed to read index.html")?;
    let header = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../templates/header.html"
    ));
    let mobile_nav = concat!(
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../templates/mobile-nav-start.html"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../templates/mobile-nav-end.html"
        ))
    );

    let output = index
        .replace("<!-- header-placeholder -->", header)
        .replace("<!-- mobile-nav-placeholder -->", mobile_nav);

    fs::File::create(index_path)
        .context("Failed to create output file")?
        .write(output.as_bytes())
        .context("Failed to write to output file")?;

    println!("Playground post-processed: staging_dir - {staging_dir:?}");

    Ok(())
}
