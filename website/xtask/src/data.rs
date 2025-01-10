use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub latest: String,
}

impl Data {
    pub fn load() -> Result<Self> {
        Ok(toml::from_str(include_str!("../../templates/data.toml"))?)
    }
}
