use anyhow::{Context, Result};
use std::{fs, io::Write, path::PathBuf};
use tera::Tera;

use crate::data::Data;

pub fn run(staging_dir: &str) -> Result<()> {
    let index_path: PathBuf = [staging_dir, "index.html"].iter().collect();

    let index = fs::read_to_string(&index_path).context("Failed to read index.html")?;

    let header_template = concat!(env!("CARGO_MANIFEST_DIR"), "/../templates/header.html");
    let mobile_nav_template = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../templates/mobile-nav-start.html"
    );

    let mut tera = Tera::default();
    tera.add_template_files([
        (header_template, Some("header")),
        (mobile_nav_template, Some("mobile_nav")),
    ])
    .context("Failed to add template files")?;

    let mut data_map = serde_json::Map::new();
    data_map.insert("data".into(), serde_json::to_value(Data::load()?)?);
    let render_context = tera::Context::from_value(serde_json::Value::Object(data_map))?;

    let header = tera.render("header", &render_context)?;
    let mut mobile_nav = tera.render("mobile_nav", &render_context)?;

    mobile_nav.push_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../templates/mobile-nav-end.html"
    )));

    let output = index
        .replace("<!-- header-placeholder -->", &header)
        .replace("<!-- mobile-nav-placeholder -->", &mobile_nav);

    fs::File::create(index_path)
        .context("Failed to create output file")?
        .write(output.as_bytes())
        .context("Failed to write to output file")?;

    println!("Playground post-processed: staging_dir - {staging_dir:?}");

    Ok(())
}
