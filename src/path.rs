use std::env;
use std::error::Error;
use std::path::PathBuf;

use pdf::read::*;

// TODO: this is based off of compiling, need a better function to set directories.
#[deprecated = "This function searches for the ENV variable: CARGO_MANIFEST_DIR. This will not work if you decide to build the binary."]
pub fn root_dir() -> Result<PathBuf, Box<dyn Error>> {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")?;

    let root_dir = PathBuf::from(manifest_dir);

    Ok(root_dir)
}

pub fn executable_dir() -> Result<PathBuf, Box<dyn Error>> {
    let executable = env::current_exe()?;

    let directory = executable.parent().ok_or("Failed to get parent directory")?;

    Ok(directory.to_path_buf())
}