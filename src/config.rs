extern crate toml;

use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub display: DisplayConfig
}

#[derive(Debug, Deserialize)]
pub struct DisplayConfig {
    pub width: u32,
    pub height: u32,
    pub max_fps: u64
}

impl Config {
    pub fn from_file(path: &Path) -> Config {
        let text = fs::read_to_string(path).unwrap();
        toml::from_str(&text).unwrap()
    }
}

