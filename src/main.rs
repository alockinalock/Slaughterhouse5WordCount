use std::path::PathBuf;

pub mod path;
pub mod pdf;

use pdf::read::*;
use path::*;

lazy_static::lazy_static! {
    pub static ref ROOT_DIR: PathBuf = root_dir().expect("Failed to get root folder");

    pub static ref PDF_DIR: PathBuf = ROOT_DIR.join("assets");
}

fn main() {
    let vec = to_vector(get_pdfs(&PDF_DIR));
    print!("{:?}", remove_all_pdf_suffixes(vec));
}
