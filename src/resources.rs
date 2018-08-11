use image;
use std;
use std::fmt::{self, Display, Formatter};
use std::fs::{self, File};
use std::io::{self, Read};
use std::path::{Path, PathBuf};

///Errors related to resource loading.
#[derive(Debug)]
pub enum ResourceError {
    ///The ResourceLoader could not find the path to the current executable.
    ExecutablePathNotFound,
    Io(io::Error),
    Image(image::ImageError),
}

impl From<io::Error> for ResourceError {
    fn from(error: io::Error) -> Self {
        ResourceError::Io(error)
    }
}

impl From<image::ImageError> for ResourceError {
    fn from(error: image::ImageError) -> Self {
        ResourceError::Image(error)
    }
}

impl Display for ResourceError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            ResourceError::ExecutablePathNotFound => {
                write!(f, "Error: Could not locate executable path.")
            }
            ResourceError::Io(error) => write!(f, "{}", error),
            ResourceError::Image(error) => write!(f, "{}", error),
        }
    }
}

///Loads and manages Resource files.
pub struct ResourceLoader {
    res_root: PathBuf,
}

impl ResourceLoader {
    ///Attempts to create a new ResourceLoader for the current folder.
    pub fn new() -> Result<ResourceLoader, ResourceError> {
        //Get path to executable
        let executable_name =
            std::env::current_exe().map_err(|_| ResourceError::ExecutablePathNotFound)?;

        //Get parent dir
        let executable_dir = executable_name
            .parent()
            .ok_or(ResourceError::ExecutablePathNotFound)?;

        //Get resources dir
        let res_root = executable_dir.join(Path::new("res"));

        Ok(ResourceLoader { res_root })
    }

    ///Returns absolute path when provided with a path relative to the "res" directory.
    fn get_path(&self, path: &Path) -> PathBuf {
        self.res_root.join(path)
    }

    ///Opens file from "res" directory
    fn get_file(&self, path: &Path) -> io::Result<File> {
        let file_path = self.get_path(path);
        fs::File::open(file_path)
    }

    ///Load image from PNG file.
    pub fn load_png(&self, path: &Path) -> Result<image::RgbaImage, ResourceError> {
        let path = self.get_path(path);

        Ok(image::open(path)?.to_rgba())
    }

    ///Load String from file.
    pub fn load_string(&self, path: &Path) -> Result<String, ResourceError> {
        //Open file
        let mut file = self.get_file(path)?;

        //Allocate string
        let mut string = String::new();

        //Read file to string
        file.read_to_string(&mut string)?;

        Ok(string)
    }
}
