use std::path::PathBuf;

pub mod path;
pub mod pdf;

use path::*;
use pdf::read::*;

lazy_static::lazy_static! {
    pub static ref ROOT_DIR: PathBuf = root_dir().expect("Failed to get root folder");

    pub static ref PDF_DIR: PathBuf = ROOT_DIR.join("assets");
}

fn main() {
    let vect = get_pdfs(&PDF_DIR).unwrap();
    println!("{:?}", remove_all_pdf_suffixes(vect));

    read_pdf(PDF_DIR.join("pdf-test-1.pdf"));
}
