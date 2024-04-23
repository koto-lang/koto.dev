use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize)]
pub struct DocsInfo {
    pub latest: String,
}

impl DocsInfo {
    pub fn get_info() -> Result<Self> {
        toml::from_str(&fs::read_to_string("content/docs/info.toml")?)
            .context("failed to read docs info")
    }
}
