use std::io::Write;
use std::path::PathBuf;
use std::fs;
use std::fs::*;
use std::error::Error;
use std::collections::HashMap;

// these 2 functions need to exist because of root problem with how we sort the hashmap
pub fn save_to_json_for_hashmap(data: &HashMap<String, usize>, path_to_saved: PathBuf) {
        let json_string = serde_json::to_string_pretty(&data).unwrap();

        let file_name = format!("{}.json", manage_file_name(path_to_saved.clone()).unwrap_or("data0".to_string()));

        let mut file = File::create(path_to_saved.join(file_name)).expect("Failed to create file");
        file.write_all(json_string.as_bytes()).expect("Failed to write to file");
}

pub fn save_to_json_for_vec(data: &Vec<(String, usize)>, path_to_saved: PathBuf) {
        let json_string = serde_json::to_string_pretty(&data).unwrap();

        let file_name = format!("{}.json", manage_file_name(path_to_saved.clone()).unwrap_or("data0".to_string()));

        let mut file = File::create(path_to_saved.join(file_name)).expect("Failed to create file");
        file.write_all(json_string.as_bytes()).expect("Failed to write to file");
}

// The nature of the for loop's implementation means this result type is needed
fn manage_file_name(saved_path: PathBuf) -> Result<String, Box<dyn Error>> {
        let mut file_names: Vec<String> = Vec::new();

        let entries = fs::read_dir(saved_path)?;

        for entry in entries {
                let current_entry = entry?;

                let file_name = current_entry.file_name().to_string_lossy().into_owned();
        
                file_names.push(file_name);
        }

        let file_num = file_names.len();

        let returned_name = format!("data{}", file_num);

        Ok(returned_name)
}