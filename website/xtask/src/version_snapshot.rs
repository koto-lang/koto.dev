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

pub fn run(new_version: &str, old_version: &str) -> Result<()> {
    let old_docs_target = format!("content/docs/{old_version}");
    let old_docs_target_path = PathBuf::from(&old_docs_target);
    let new_docs_target = format!("content/docs/latest");
    let new_docs_target_path = PathBuf::from(&new_docs_target);
    let playground_target = format!("static/play-{new_version}");
    let playground_target_path = PathBuf::from(&playground_target);
    let copy_options = fs_extra::dir::CopyOptions::new().copy_inside(true);

    if old_docs_target_path.exists() {
        fs::remove_dir_all(&old_docs_target_path)
            .with_context(|| format!("failed to remove '{old_docs_target}'"))?;
    }
    fs_extra::copy_items(
        &["content/docs/latest"],
        &old_docs_target_path,
        &copy_options,
    )
    .with_context(|| format!("failed to copy docs to '{old_docs_target}'"))?;
    println!("Latest docs copied to '{old_docs_target}'");

    if new_docs_target_path.exists() {
        fs::remove_dir_all(&new_docs_target_path)
            .with_context(|| format!("failed to remove '{new_docs_target}'"))?;
    }
    fs_extra::copy_items(
        &["content/docs/latest"],
        &new_docs_target_path,
        &copy_options,
    )
    .with_context(|| format!("failed to copy docs to '{new_docs_target}'"))?;
    println!("New docs copied to '{new_docs_target}'");

    if playground_target_path.exists() {
        fs::remove_dir_all(&playground_target_path)
            .with_context(|| format!("failed to remove '{playground_target}'"))?;
    }
    fs_extra::copy_items(&["static/play"], &playground_target_path, &copy_options)
        .with_context(|| format!("Error while copying the playground to {playground_target}",))?;
    println!("Playground copied to '{playground_target}'",);

    // Post-process the copied docs
    let playground_link_search = "example_playground_link()";
    let playground_link_replacement =
        format!("example_playground_link(version = \"{new_version}\")");
    for f in WalkDir::new(&new_docs_target_path) {
        let f = f.with_context(|| format!("error while traversing {new_docs_target}",))?;
        let path = f.path();
        if !path.is_file() {
            continue;
        }

        if path.file_name() == Some(OsStr::new(".gitignore")) {
            fs::remove_file(path)?;
        }

        if path.parent() == Some(&new_docs_target_path)
            && path.file_name() == Some(OsStr::new("_index.md"))
        {
            update_index_title(path, new_version)?;
            continue;
        }

        if path.extension() == Some(OsStr::new("md")) {
            search_and_replace_in_file(path, playground_link_search, &playground_link_replacement)?;
            println!("Updated playground links in '{}'", path.to_string_lossy());
        }
    }

    // Post-process the copied playground
    let playground_index = PathBuf::from(&format!("{playground_target}/index.html"));
    search_and_replace_in_file(
        &playground_index,
        "/play/",
        &format!("/play-{new_version}/"),
    )?;
    println!(
        "Updated playground references in '{}'",
        playground_index.to_string_lossy()
    );

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
