use ron::de;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub display: DisplayConfig,
}

#[derive(Debug, Deserialize)]
pub struct DisplayConfig {
    pub width: u32,
    pub height: u32,
    pub max_fps: u64,
}

impl Config {
    pub fn from_file(path: &Path) -> Result<Config, String> {
        let text = fs::read_to_string(path).unwrap();
        match de::from_str::<Config>(&text) {
            Ok(config) => return Ok(config),
            Err(error) => return Err(format!("Could not load config file from {:?}\nError: {}", path, error)),
        }
    }
}
