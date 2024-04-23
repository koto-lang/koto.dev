use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DocsInfo {
    pub latest: String,
}

impl DocsInfo {
    pub fn get_info() -> Self {
        let info = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../content/docs/info.toml"
        ));
        toml::from_str(info).unwrap()
    }
}
