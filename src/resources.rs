use image;

use std;
use std::ffi;
use std::fs::{self, File};
use std::io::{self, Read};
use std::path::{Path, PathBuf};

#[derive(Debug)]
///Errors related to resource loading.
pub enum Error {
    ///The ResourceLoader could not find the path to the current executable.
    ExecutablePathNotFound,
    FileContainsNullByte(PathBuf),
    Io(io::Error),
    Image(image::ImageError),
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::Io(error)
    }
}

impl From<image::ImageError> for Error {
    fn from(error: image::ImageError) -> Self {
        Error::Image(error)
    }
}

impl From<Error> for String {
    fn from(error: Error) -> Self {
        match error {
            Error::ExecutablePathNotFound => format!("Error: Could not locate executable path."),
            Error::FileContainsNullByte(path) => format!("Error: File \"{:?}\" contains a null byte.", path),
            Error::Io(error) => format!("{}", error),
            Error::Image(error) => format!("{}", error),
        }
    }
}

///Loads and manages Resource files.
pub struct ResourceLoader {
    res_root: PathBuf,
}

impl ResourceLoader {
    ///Attempts to create a new ResourceLoader for the current folder.
    pub fn new() -> Result<ResourceLoader, Error> {
        //Get path to executable
        let executable_name = std::env::current_exe()
            .map_err(|_| Error::ExecutablePathNotFound)?;

        //Get parent dir
        let executable_dir = executable_name.parent().ok_or(Error::ExecutablePathNotFound)?;

        //Get resources dir
        let res_dir = executable_dir.join(Path::new("res"));

        Ok(ResourceLoader { res_root: res_dir.into() })
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
    pub fn load_png(&self, path: &Path) -> Result<image::RgbaImage, Error> {
        let path = self.get_path(path);
        match image::open(path) {
            Ok(img) => Ok(img.to_rgba()),
            Err(error) => Err(Error::Image(error)),
        }
    }

    ///Load String from file.
    pub fn load_string(&self, path: &Path) -> Result<String, Error> {
        //Open file
        let mut file = self.get_file(path)?;

        //Allocate string
        let mut string = String::new();

        //Read file to string
        file.read_to_string(&mut string)?;

        Ok(string)
    }

    ///Load CString from file, making sure that it doesn't contain a null byte.
    pub fn load_cstring(&self, path: &Path) -> Result<ffi::CString, Error> {
        //Open file
        let mut file = self.get_file(path)?;

        //Allocate buffer for contents
        let mut buffer: Vec<u8> = Vec::with_capacity(
            file.metadata()?.len() as usize + 1
        );

        //Read file into buffer
        file.read_to_end(&mut buffer)?;

        //If buffer contains null byte, return error
        if buffer.iter().find(|i| **i == 0).is_some() {
            return Err(Error::FileContainsNullByte(path.into()));
        }

        //Otherwise, return CString from buffer
        Ok(unsafe { ffi::CString::from_vec_unchecked(buffer) })
    }
}
