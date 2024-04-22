//! Takes a versioned snapshot of the 'next' docs folder and playground
//!
//! The version string will be appended to /play urls found in all markdown files in the snapshot.

use std::{
    ffi::OsStr,
    fs::{self, File},
    io::{BufRead, BufReader, Write},
    path::{Path, PathBuf},
};

use anyhow::{bail, Context, Result};
use walkdir::WalkDir;

pub fn run(version: &str) -> Result<()> {
    let docs_target = format!("content/docs/{version}");
    let docs_target_path = PathBuf::from(&docs_target);
    let playground_target = format!("static/play-{version}");
    let playground_target_path = PathBuf::from(&playground_target);
    let copy_options = fs_extra::dir::CopyOptions::new().content_only(true);

    // Copy next -> version
    if docs_target_path.exists() {
        fs::remove_dir_all(&docs_target_path)
            .with_context(|| format!("failed to remove '{docs_target}'"))?;
    }
    fs::create_dir(&docs_target_path)
        .with_context(|| format!("failed to create '{docs_target}'"))?;
    fs_extra::dir::copy("content/docs/next", &docs_target, &copy_options)
        .with_context(|| format!("failed to copy docs to '{docs_target}'"))?;
    println!("Docs copied from docs/next to '{docs_target}'");

    // Copy the playground
    if playground_target_path.exists() {
        fs::remove_dir_all(&playground_target_path)
            .with_context(|| format!("failed to remove '{playground_target}'"))?;
    }
    fs_extra::dir::copy("static/play", &playground_target_path, &copy_options)
        .with_context(|| format!("Error while copying the playground to {playground_target}",))?;
    println!("Playground copied to '{playground_target}'",);

    // Post-process the copied docs
    let playground_link_search = "example_playground_link()";
    let playground_link_replacement = format!("example_playground_link(version = \"{version}\")");
    for f in WalkDir::new(&docs_target_path) {
        let f = f.with_context(|| format!("error while traversing {docs_target}",))?;
        let path = f.path();
        if !path.is_file() {
            continue;
        }

        if path.file_name() == Some(OsStr::new(".gitignore")) {
            fs::remove_file(path)?;
        }

        if path.parent() == Some(&docs_target_path)
            && path.file_name() == Some(OsStr::new("_index.md"))
        {
            update_index_title(path, version)?;
            continue;
        }

        if path.extension() == Some(OsStr::new("md")) {
            search_and_replace_in_file(path, playground_link_search, &playground_link_replacement)?;
            println!("Updated playground links in '{}'", path.to_string_lossy());
        }
    }

    // Post-process the copied playground
    let playground_index = PathBuf::from(&format!("{playground_target}/index.html"));
    search_and_replace_in_file(&playground_index, "/play/", &format!("/play-{version}/"))?;
    println!(
        "Updated playground references in '{}'",
        playground_index.to_string_lossy()
    );

    // Write the latest version in docs/info.toml
    let info_path = "content/docs/info.toml";
    let mut info_file =
        File::create(info_path).with_context(|| format!("failed to create file at {info_path}"))?;
    writeln!(info_file, "latest = \"{version}\"")?;
    println!("docs/info.toml updated");

    Ok(())
}

fn update_index_title(path: &Path, version: &str) -> Result<()> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut result = Vec::new();
    let mut line_replaced = false;

    for line in reader.lines() {
        let line =
            line.with_context(|| format!("couldn't read from '{}'", path.to_string_lossy()))?;

        if line.starts_with("title = ") {
            result.push(format!("title = \"{version}\""));
            line_replaced = true;
        } else {
            result.push(line);
        }
    }

    if !line_replaced {
        bail!("Missing title entry in '{}'", path.to_string_lossy());
    }

    let mut file = File::create(path)
        .with_context(|| format!("failed to create file at '{}'", path.to_string_lossy()))?;
    for line in result {
        writeln!(file, "{}", line)?;
    }

    println!("Updated title in '{}'", path.to_string_lossy());

    Ok(())
}

fn search_and_replace_in_file(path: &Path, pattern: &str, replacement: &str) -> Result<()> {
    let contents = fs::read_to_string(path)
        .with_context(|| format!("failed to read from '{}'", path.to_string_lossy()))?;
    let result = contents.replace(pattern, replacement);

    let mut file = File::create(path)?;
    file.write_all(result.as_bytes())?;

    Ok(())
}
