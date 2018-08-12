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
