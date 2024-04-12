use std::path::PathBuf;
use std::error::Error;


pub fn root_dir() -> Result<PathBuf, Box<dyn Error>> {
	let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")?;

	let root_dir = PathBuf::from(manifest_dir);
	
	Ok(root_dir)
}