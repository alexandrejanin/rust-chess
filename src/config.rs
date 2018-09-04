use cuivre::resources::Loadable;
use ron;
use std::{error, fmt};

#[derive(Debug)]
pub enum ConfigError {
    Ron(ron::de::Error),
}

impl error::Error for ConfigError {}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Could not load Config: ")?;
        match self {
            ConfigError::Ron(error) => write!(f, "{}", error),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub video: VideoConfig,
}

#[derive(Debug, Deserialize)]
pub struct VideoConfig {
    pub width: u32,
    pub height: u32,
    pub vsync: bool,
}

impl Loadable for Config {
    type LoadOptions = ();
    type LoadResult = Result<Self, ConfigError>;

    fn load_from_bytes(data: &[u8], _options: ()) -> Result<Self, ConfigError> {
        ron::de::from_bytes::<Self>(data).map_err(ConfigError::Ron)
    }
}
