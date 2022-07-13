use std::{fs, io::Write, path::PathBuf};

fn main() {
    let staging_dir = std::env::args()
        .skip(1)
        .next()
        .expect("Missing staging dir arg");
    let index_path: PathBuf = [staging_dir, "index.html".to_string()].iter().collect();

    let index = fs::read_to_string(&index_path).expect("Failed to read index contents");
    let header = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../templates/header.html"
    ));
    let mobile_nav = concat!(
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../templates/mobile-nav-start.html"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../templates/mobile-nav-end.html"
        ))
    );

    let output = index
        .replace("<!-- header-placeholder -->", header)
        .replace("<!-- mobile-nav-placeholder -->", &mobile_nav);

    fs::File::create(index_path)
        .expect("Failed to open output path")
        .write(output.as_bytes())
        .expect("Failed to write to output");
}
