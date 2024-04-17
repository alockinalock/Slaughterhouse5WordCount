use pdf::read::*;
use std::error::Error;
use std::path::PathBuf;

pub fn root_dir() -> Result<PathBuf, Box<dyn Error>> {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")?;

    let root_dir = PathBuf::from(manifest_dir);

    Ok(root_dir)
}

pub fn join_all(pdf_dir: PathBuf) -> Vec<PathBuf> {
    let mut pdfs: Vec<PathBuf> = Vec::new();

    // FIXME: using unwrap here is a really bad idea. get on it and fix it.
    let pdf_names = get_pdfs(&pdf_dir).unwrap();

    for pdf in pdf_names {
        let complete_path = pdf_dir.join(pdf);
        pdfs.push(complete_path);
    }

    pdfs
}
