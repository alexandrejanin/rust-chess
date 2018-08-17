use cuivre::resources::Loadable;
use ron;
use std::{error, fmt, io};

#[derive(Debug)]
pub enum ConfigError {
    Io(io::Error),
    Ron(ron::de::Error),
}

impl error::Error for ConfigError {}

impl From<io::Error> for ConfigError {
    fn from(error: io::Error) -> Self {
        ConfigError::Io(error)
    }
}

impl From<ron::de::Error> for ConfigError {
    fn from(error: ron::de::Error) -> Self {
        ConfigError::Ron(error)
    }
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Could not load Config: ")?;
        match self {
            ConfigError::Io(error) => write!(f, "{}", error),
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
    type LoadError = ConfigError;

    fn load_from_bytes(data: &[u8], _options: ()) -> Result<Self, ConfigError> {
        ron::de::from_bytes::<Self>(data).map_err(ConfigError::Ron)
    }
}
