use std::path::PathBuf;

use resources;

pub mod manager;
pub mod shaders;

#[derive(Debug)]
pub enum Error {
    ResourceError { path: PathBuf, message: String },
    ShaderCompilationFailed { message: String },
    ProgramLinkingFailed { message: String },
}

impl From<Error> for String {
    fn from(error: Error) -> Self {
        match error {
            Error::ResourceError { path, message } => format!("Error: Could not load resource \"{:?}\"\n{}", path, message),
            Error::ShaderCompilationFailed { message } => format!("Error: Shader compilation failed\n{}", message),
            Error::ProgramLinkingFailed { message } => format!("Error: Program linking failed\n{}", message),
        }
    }
}
