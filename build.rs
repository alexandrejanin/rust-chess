extern crate fs_extra;

use std::env;
use std::path::PathBuf;

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    // locate executable path even if the project is in workspace
    let executable_path = manifest_dir
        .join("target")
        .join(env::var("PROFILE").unwrap());

    //Set copy options for directories
    let dir_options = fs_extra::dir::CopyOptions {
        overwrite: true,
        skip_exist: false,
        buffer_size: 64_000,
        copy_inside: true,
        depth: 0,
    };

    //Set copy options for files
    let file_options = fs_extra::file::CopyOptions {
        overwrite: true,
        skip_exist: false,
        buffer_size: 64_000,
    };

    //Remove res directory
    fs_extra::dir::remove(&executable_path.join("res"))
        .expect("Error: could not remove 'res' directory.");

    //Copy res directory
    if let Err(error) = fs_extra::dir::copy(
        &manifest_dir.join("res"),
        &executable_path.join("res"),
        &dir_options,
    ) {
        panic!("Error: Could not copy 'res' folder.\n{}", error)
    }

    //Copy DLL
    if let Err(error) = fs_extra::file::copy(
        &manifest_dir.join("bin/SDL2.dll"),
        &executable_path.join("SDL2.dll"),
        &file_options,
    ) {
        panic!("Error: Could not copy 'SDL2.dll'.\n{}", error)
    }
}
