use std::fs::File;
use std::io::Write;
use std::path::Path;
use tauri::State;
use tokio::sync::Mutex;
use crate::app_state::AppState;

// Command to write the config.json file with the provided JSON data
#[tauri::command]
pub fn write_config(json_data: String) -> Result<(), String> {
    let config_path = Path::new("config.json");
    let mut file = File::create(&config_path).map_err(|err| err.to_string())?;

    file.write_all(json_data.as_bytes())
        .map_err(|err| err.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn get_env(name: &str) -> String {
    std::env::var(String::from(name)).unwrap_or_else(|_| String::from(""))
}

// Add any other commands here as needed

pub fn cleanup(state: &State<'_, Mutex<AppState>>) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let mut sst = rt.block_on(state.lock());
    sst.cleanup();
}
