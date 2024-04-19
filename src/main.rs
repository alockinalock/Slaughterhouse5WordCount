use std::path::PathBuf;

pub mod path;
pub mod pdf;

use path::*;
use pdf::parse::*;
use pdf::read::*;

use std::io;

extern crate regex;

lazy_static::lazy_static! {
    pub static ref ROOT_DIR: PathBuf = root_dir().expect("Failed to get root folder");

    pub static ref PDF_DIR: PathBuf = ROOT_DIR.join("assets");

    pub static ref SAVE_DIR: PathBuf = ROOT_DIR.join("saved");
}

struct Config {
    sorted: bool,
    saved: bool,
}

fn main() {
    let mut userConfig = Config {
        sorted: false,
        saved: false,
    };

    // Created saved folder if it doesn't exist
    if !SAVE_DIR.exists() {
        std::fs::create_dir(SAVE_DIR.as_path()).expect("Failed to create saved folder");
    }

    // this line may panic
    let pdfs = get_pdfs(&PDF_DIR).unwrap();

    gui_list_pdfs_ordered(&pdfs);

    println!("Enter the number which represents the PDF document you would like to use.");

    let mut temp_input_holder: String = String::new();
    io::stdin().read_line(&mut temp_input_holder).expect("Failed to read input");

    let temp_int_holder: usize = temp_input_holder.trim().parse().unwrap();

    let selected_pdf: PathBuf = PDF_DIR.join(pdfs.get(temp_int_holder).unwrap().clone());

    // Ask user if they want it to be sorted
    println!("Would you like for the list to be sorted? [This will apply to both printing and saving]");
    io::stdin().read_line(&mut temp_input_holder).expect("Failed to read input");

    if temp_input_holder.trim().to_ascii_lowercase().starts_with("y") {
        userConfig.sorted = true;
    }
    else {
        userConfig.sorted = false;
    }

    // Ask user if they want it to be saved to JSON
    println!("Would you like for the list to be saved? [This will save as JSON]");
    io::stdin().read_line(&mut temp_input_holder).expect("Failed to read input");

    if temp_input_holder.trim().to_ascii_lowercase().starts_with("y") {
        userConfig.saved = true;
    }
    else {
        userConfig.saved = false;
    }

    let data: String = read_pdf(&selected_pdf);

    println!("*** Reading selected pdf");

    let word_counts;
    if userConfig.sorted {
        word_counts = sort_by_instances_hash(regex_parse_for_words(&data));
    }
    else {
        word_counts = regex_parse_for_words(&data);
    }


    let sorted_word_counts = sort_by_instances(word_counts);

    let reversed_swc = sorted_word_counts.iter().rev();

    for (word, count) in reversed_swc {
        println!("{}: {}", word, count);
    }
}

fn gui_list_pdfs_ordered(vec: &Vec<String>) {
    let mut iterator_count = 0;
    for item in vec {
        println!("{}) {}", iterator_count, item);
        iterator_count += 1;
    }
}