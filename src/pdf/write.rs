use std::path::PathBuf;
use std::fs;
use std::error::Error;


pub fn save_to_json() {

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