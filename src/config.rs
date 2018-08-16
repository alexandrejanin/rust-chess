use cuivre::resources::Loadable;
use ron;

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
    type LoadResult = Result<Self, ron::de::Error>;

    fn load(data: &[u8], _options: ()) -> Result<Self, ron::de::Error> {
        ron::de::from_bytes(data)
    }
}
