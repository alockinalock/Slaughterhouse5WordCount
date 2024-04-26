use std::error::Error;
use std::path::PathBuf;

// TODO: this is based off of compiling, need a better function to set directories.
pub fn root_dir() -> Result<PathBuf, Box<dyn Error>> {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")?;

    let root_dir = PathBuf::from(manifest_dir);

    Ok(root_dir)
}