use ron::de;
use std::path::Path;

use resources::ResourceLoader;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub display: DisplayConfig,
}

#[derive(Debug, Deserialize)]
pub struct DisplayConfig {
    pub width: u32,
    pub height: u32,
    pub vsync: bool,
}

impl Config {
    ///Load config from .ron file.
    pub fn from_file(resource_loader: &ResourceLoader, path: &Path) -> Result<Config, String> {
        let text = resource_loader.load_string(path)
                                  .map_err(|error| format!("Could not load config from {:?}\nError: {}", path, error))?;

        de::from_str(&text)
            .map_err(|error| format!("Could not load config from {:?}\nError: {}", path, error))
    }
}
