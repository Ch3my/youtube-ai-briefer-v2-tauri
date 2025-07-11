use serde_json::{json, Value};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use tauri::{AppHandle, Manager};

#[tauri::command]
pub fn write_config(app_handle: AppHandle, json_data: String) -> Result<(), String> {
    let app_data_base_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data directory: {}", e))?;

    // Construct the full path to config.json within the app data directory
    let config_path = app_data_base_dir.join("config.json");

    // Check if the file already exists
    if config_path.exists() {
        println!("Config file already exists. Overwriting...");

        // Open the file in write mode to overwrite its contents
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true) // Overwrite the file
            .open(&config_path)
            .map_err(|err| format!("Failed to open config file for writing: {}", err))?;

        file.write_all(json_data.as_bytes())
            .map_err(|err| format!("Failed to write to config file: {}", err))?;
    } else {
        println!("Config file does not exist. Creating a new one...");

        // Create the file if it doesn't exist
        let mut file = File::create(&config_path)
            .map_err(|err| format!("Failed to create config file: {}", err))?;

        file.write_all(json_data.as_bytes())
            .map_err(|err| format!("Failed to write to new config file: {}", err))?;
    }

    println!("Configuration has been written successfully.");
    Ok(())
}

#[tauri::command]
pub fn read_config(app_handle: AppHandle) -> Result<Value, String> {
    // Get the base application data directory
    let app_data_base_dir = app_handle.path().app_data_dir()
        .map_err(|e| format!("Failed to get app data directory: {}", e))?;

    // Construct the full path to config.json within the app data directory
    let config_path = app_data_base_dir.join("config.json");

    // Check if config.json exists, and if not, create it with default values
    if !config_path.exists() {
        let default_config = json!({
            "resumeModel": "gpt-4o-mini",
            "resumeChunkSize": 10000,
            "condensaModel": "gpt-4o-mini",
            "ragModel": "gpt-4o-mini",
            "ragSearchType": "similarity",
            "ragSearchK": 5,
            "ragChunkSize": 1000,
            "useWhisper": "no"
        });

        let mut file = File::create(&config_path).map_err(|err| err.to_string())?;
        file.write_all(default_config.to_string().as_bytes())
            .map_err(|err| err.to_string())?;
    }

    // Open the file for reading after ensuring it exists
    let mut file = OpenOptions::new()
        .read(true)
        .open(&config_path)
        .map_err(|err| err.to_string())?;

    // Read the file content into a string
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|err| err.to_string())?;

    // Parse the string content as JSON
    let json_data: Value = serde_json::from_str(&contents).map_err(|err| err.to_string())?;

    Ok(json_data)
}

#[tauri::command]
pub fn get_env(name: &str) -> String {
    std::env::var(String::from(name)).unwrap_or_else(|_| String::from(""))
}

#[tauri::command]
pub fn get_log_file_location(app_handle: AppHandle) -> Result<String, String> {
    // Get the base application data directory
    let app_data_base_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data directory: {}", e))?;

    // Construct the full path to log.txt
    let log_file_path = app_data_base_dir.join("log.txt");

    // Return the path as a string
    Ok(log_file_path.to_string_lossy().into_owned())
}
