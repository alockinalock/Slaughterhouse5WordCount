use std::error::Error;
use std::path::PathBuf;

pub fn root_dir() -> Result<PathBuf, Box<dyn Error>> {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")?;

    let root_dir = PathBuf::from(manifest_dir);

    Ok(root_dir)
}