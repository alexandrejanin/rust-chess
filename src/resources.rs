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
    FileContainsNullByte { path: PathBuf },
    Io(io::Error),
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::Io(error)
    }
}

impl From<Error> for String {
    fn from(error: Error) -> Self {
        match error {
            Error::ExecutablePathNotFound => format!("Error: Could not locate executable path."),
            Error::FileContainsNullByte { path } => format!("Error: File \"{:?}\" contains a null byte.", path),
            Error::Io(err) => format!("{:?}", err),
        }
    }
}


///Loads and manages Resource files.
pub struct ResourceLoader {
    root_path: PathBuf,
}

impl ResourceLoader {
    ///Attempts to create a new ResourceLoader for the current folder.
    pub fn new() -> Result<ResourceLoader, Error> {
        //Get path to executable
        let executable_name = std::env::current_exe()
            .map_err(|_| Error::ExecutablePathNotFound)?;

        //Get parent dir
        let executable_dir = executable_name.parent().ok_or(Error::ExecutablePathNotFound)?;

        Ok(ResourceLoader { root_path: executable_dir.into() })
    }

    ///Load `String` from file.
    pub fn load_string(&self, path: &Path) -> io::Result<String> {
        //Open file
        let mut file = self.get_file(path)?;

        //Allocate string
        let mut string = String::new();

        //Read file to string
        file.read_to_string(&mut string)?;

        Ok(string)
    }

    ///Load `CString` from file, making sure that it doesn't contain a `null` byte.
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
            return Err(Error::FileContainsNullByte { path: path.into() });
        }

        //Otherwise, return CString from buffer
        Ok(unsafe { ffi::CString::from_vec_unchecked(buffer) })
    }

    ///Opens file from root path
    fn get_file(&self, path: &Path) -> io::Result<File> {
        let file_path = self.root_path.join(path);
        println!("Loading: {:?}", file_path);
        fs::File::open(file_path)
    }
}