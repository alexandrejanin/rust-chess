use resources;

pub mod manager;
mod data;
mod shaders;

#[derive(Debug)]
pub enum Error {
    ResourceError(resources::Error),
    ShaderCompilationFailed(String),
    ProgramLinkingFailed(String),
}

impl From<resources::Error> for Error {
    fn from(error: resources::Error) -> Self {
        Error::ResourceError(error)
    }
}

impl From<Error> for String {
    fn from(error: Error) -> Self {
        match error {
            Error::ResourceError(error) => format!("Error: Could not load resource\n{}", String::from(error)),
            Error::ShaderCompilationFailed(message) => format!("Error: Shader compilation failed\n{}", message),
            Error::ProgramLinkingFailed(message) => format!("Error: Program linking failed\n{}", message),
        }
    }
}
