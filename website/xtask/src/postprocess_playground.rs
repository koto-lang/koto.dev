use anyhow::{Context, Result};
use std::{fs, io::Write, path::PathBuf};

use crate::docs_info::DocsInfo;

pub fn run(staging_dir: &str) -> Result<()> {
    let index_path: PathBuf = [staging_dir, "index.html"].iter().collect();

    let index = fs::read_to_string(&index_path).context("Failed to read index.html")?;
    let header = get_header()?;
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
        .replace("<!-- header-placeholder -->", &header)
        .replace("<!-- mobile-nav-placeholder -->", mobile_nav);

    fs::File::create(index_path)
        .context("Failed to create output file")?
        .write(output.as_bytes())
        .context("Failed to write to output file")?;

    println!("Playground post-processed: staging_dir - {staging_dir:?}");

    Ok(())
}

fn get_header() -> Result<String> {
    // The header is a Zola template, but the playground isn't rendered with Zola,
    // so it needs to be patched manually.
    let header_template = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../templates/header.html"
    ));
    let latest = DocsInfo::get_info().latest;
    let mut result = String::with_capacity(header_template.len());
    for line in header_template.lines() {
        if line.starts_with("{%") {
            continue;
        }
        result.push_str(&line.replace(r"{{latest}}", &latest));
    }
    Ok(result)
}
