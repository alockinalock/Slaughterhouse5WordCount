use std::error::Error;
use std::fs;
use std::path::PathBuf;

pub fn get_pdfs(folder_path: &PathBuf) -> Result<Vec<String>, Box<dyn Error>> {
    let mut names: Vec<String> = Vec::new();

    let entries = fs::read_dir(folder_path)?;

    for entry in entries {
        let current_entry = entry?;

        let file_name = current_entry.file_name().to_string_lossy().into_owned();

        if file_name.to_lowercase().ends_with(".pdf") {
            names.push(file_name.to_string());
        }
    }

    Ok(names)
}

pub fn remove_pdf_suffix(file_name: String) -> String {
    let new_file_name: String;
    if file_name.to_lowercase().ends_with(".pdf") {
        new_file_name = file_name[..file_name.len() - 4].to_string();
    } else {
        new_file_name = file_name;
    }
    new_file_name
}

pub fn remove_all_pdf_suffixes(vec: Vec<String>) -> Vec<String> {
    let mut new_vec: Vec<String> = Vec::new();
    for file in vec {
        new_vec.push(remove_pdf_suffix(file));
    }
    new_vec
}

pub fn read_pdf(path: PathBuf) -> String {
  let bytes = std::fs::read(path).unwrap();
  let out = pdf_extract::extract_text_from_mem(&bytes).unwrap();
  out
}