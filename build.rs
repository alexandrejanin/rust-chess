extern crate walkdir;

use std::env;
use std::fs::{self, DirBuilder};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    // locate executable path even if the project is in workspace
    let executable_path = manifest_dir
        .join("target")
        .join(env::var("PROFILE").unwrap());

    //Copy res files
    copy(
        &manifest_dir.join("res"),
        &executable_path.join("res"),
    );

    //Copy DLL
    copy(
        &manifest_dir.join("bin/SDL2.dll"),
        &executable_path.join("SDL2.dll"),
    );
}

///Copies file or folder from one place to another.
fn copy(from: &Path, to: &Path) {
    let from_path: PathBuf = from.into();
    let to_path: PathBuf = to.into();

    for entry in WalkDir::new(from_path.clone()) {
        let entry = entry.unwrap();

        if let Ok(rel_path) = entry.path().strip_prefix(&from_path) {
            let target_path = to_path.join(rel_path);

            if entry.file_type().is_dir() {
                DirBuilder::new()
                    .recursive(true)
                    .create(target_path).expect("failed to create target dir");
            } else {
                fs::copy(entry.path(), &target_path).expect("failed to copy");
            }
        }
    }
}
