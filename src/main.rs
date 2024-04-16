use std::path::PathBuf;

pub mod path;
pub mod pdf;

use path::*;
use pdf::parse::*;
use pdf::read::*;

extern crate regex;

lazy_static::lazy_static! {
    pub static ref ROOT_DIR: PathBuf = root_dir().expect("Failed to get root folder");

    pub static ref PDF_DIR: PathBuf = ROOT_DIR.join("assets");
}

fn main() {
    // let vect = get_pdfs(&PDF_DIR).unwrap();
    // println!("{:?}", remove_all_pdf_suffixes(vect));

    let path: PathBuf = PDF_DIR.join("pdf-test-1.pdf");

    let data: String = read_pdf(path);

    let word_counts = parse_for_words(&data);

    // let sorted_word_counts = sort_by_instances(word_counts);

    // for (word, count) in &sorted_word_counts {
    //     println!("{}: {}", word, count);
    // }

    for (word, count) in &word_counts {
        println!("{}: {}", word, count);
    }
}
