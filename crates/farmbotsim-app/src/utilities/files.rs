use farmbotsim_core::logger::log_error_and_panic;
use serde::Serialize;
use serde_json::to_string_pretty;
use std::{
    error::Error,
    fs::{self, File},
    io::Write,
};

/// Get json files that are in folder with path.
pub fn get_json_files_in_folder(path: &str) -> Vec<String> {
    let mut json_files = Vec::new();

    let entries = fs::read_dir(path).unwrap_or_else(|e| {
        let msg = format!("Failed to read directory {path}: {e}");
        log_error_and_panic(&msg);
    });

    for entry in entries {
        let entry = entry.unwrap_or_else(|e| {
            let msg = format!("Failed to read entry in {path}: {e}");
            log_error_and_panic(&msg);
        });

        let path = entry.path();
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("json") {
            if let Some(file_name) = path.file_name().and_then(|s| s.to_str()) {
                json_files.push(file_name.to_string());
            }
        }
    }

    json_files
}

/// Get folders that are in folder with path.
pub fn get_folders_in_folder(path: &str) -> Vec<String> {
    let mut folders = Vec::new();

    let entries = fs::read_dir(path).unwrap_or_else(|e| {
        let msg = format!("Failed to read directory {path}: {e}");
        log_error_and_panic(&msg);
    });

    for entry in entries {
        let entry = entry.unwrap_or_else(|e| {
            let msg = format!("Error reading entry in {path}: {e}");
            log_error_and_panic(&msg);
        });

        let path = entry.path();
        if path.is_dir() {
            if let Some(folder_name) = path.file_name().and_then(|s| s.to_str()) {
                folders.push(folder_name.to_string());
            }
        }
    }

    folders
}

/// Saves data to file with path.
pub fn save_as_json<T: Serialize>(data: &T, file_path: &str) -> Result<(), Box<dyn Error>> {
    let json = to_string_pretty(data)?;

    let mut file = File::create(file_path)?;
    file.write_all(json.as_bytes())?;

    println!("Successfully saved to {file_path}");
    Ok(())
}
