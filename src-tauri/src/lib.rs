// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod commands;
use chrono::Local;
use std::env;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::os::windows::process::CommandExt;
use std::path::PathBuf;
use std::process::Command;
use tauri::Manager;
use tauri_plugin_shell::process::CommandEvent;
use tauri_plugin_shell::ShellExt; // Required for app.handle() and app_handle.path()

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                // --- START: Simplified Log file path ---
                let app_data_base_dir = match handle.path().app_data_dir() {
                    Ok(path) => path,
                    Err(e) => {
                        eprintln!("ERROR: Failed to get app data directory: {}", e);
                        // Fallback: Use the directory of the current executable.
                        // This fallback is generally NOT recommended for production logs
                        // due to potential permission issues, but it's a last resort.
                        env::current_exe()
                            .unwrap_or_else(|_| PathBuf::from("."))
                            .parent()
                            .unwrap_or(&PathBuf::from("."))
                            .to_path_buf()
                    }
                };

                // Define the full path for log.txt directly in the app data directory
                let log_file_path = app_data_base_dir.join("log.txt");

                // Ensure the parent directory for the log file exists (which is app_data_base_dir itself)
                // This is crucial! `OpenOptions::create(true)` only creates the *file*, not parent directories.
                if let Some(parent_dir) = log_file_path.parent() {
                    if let Err(e) = fs::create_dir_all(parent_dir) {
                        eprintln!(
                            "ERROR: Failed to create parent directory for log file '{:?}': {}",
                            parent_dir, e
                        );
                        // If we can't create the parent directory, trying to open the file will fail.
                        // Panic or handle this critical error as appropriate for your application.
                        panic!("FATAL: Cannot proceed without writable log directory.");
                    }
                } else {
                    eprintln!(
                        "ERROR: Log file path has no parent directory: {:?}",
                        log_file_path
                    );
                    panic!("FATAL: Invalid log file path structure.");
                }

                // Print the resolved path for debugging
                eprintln!("Attempting to open log file at: {:?}", log_file_path);

                // Open the log file in append mode, create if it doesn't exist
                let mut log_file = match OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(&log_file_path)
                {
                    Ok(file) => file,
                    Err(e) => {
                        eprintln!(
                            "FATAL ERROR: Failed to open log file '{:?}': {}",
                            log_file_path, e
                        );
                        // If file opening fails, subsequent logging attempts will also fail.
                        // This is a critical error for a logging setup.
                        panic!(
                            "FATAL: Failed to open log file at {:?}: {}",
                            log_file_path, e
                        );
                    }
                };
                // --- END: Simplified Log file path ---

                // Convert app_data_base_dir to a string to pass as an argument
                let app_data_dir_str = match app_data_base_dir.to_str() {
                    Some(s) => s.to_string(),
                    None => {
                        eprintln!("ERROR: Failed to convert app_data_base_dir to string.");
                        // Handle this error appropriately, perhaps by using a default or panicking
                        // For this example, we'll use an empty string, but this might not be ideal
                        String::new()
                    }
                };

                let sidecar_command = handle.shell().sidecar("ai-brain")
                .unwrap()
                .args(&[app_data_dir_str]); 
                let (mut rx, _child) = sidecar_command.spawn().unwrap();

                while let Some(event) = rx.recv().await {
                    // Get current local datetime
                    let now = Local::now();
                    let timestamp = now.format("%Y-%m-%d %H:%M:%S").to_string(); // Format as YYYY-MM-DD HH:MM:SS

                    match event {
                        CommandEvent::Stdout(line_bytes) => {
                            let line = String::from_utf8_lossy(&line_bytes);
                            let cleaned_line = strip_non_printable_chars(&line);
                            println!("[{}] AI Brain stdout: {}", timestamp, line);
                            if let Err(e) = writeln!(
                                log_file,
                                "[{}] AI Brain stdout: {}",
                                timestamp, cleaned_line
                            ) {
                                eprintln!("ERROR: Failed to write to log file (stdout): {}", e);
                            }
                        }
                        CommandEvent::Stderr(line_bytes) => {
                            let line = String::from_utf8_lossy(&line_bytes);
                            let cleaned_line = strip_non_printable_chars(&line);
                            println!("[{}] AI Brain stderr: {}", timestamp, line);
                            if let Err(e) = writeln!(
                                log_file,
                                "[{}] AI Brain stderr: {}",
                                timestamp, cleaned_line
                            ) {
                                eprintln!("ERROR: Failed to write to log file (stderr): {}", e);
                            }
                        }
                        _ => {}
                    }
                }
            });

            Ok(())
        })
        .on_window_event(|window, event| match event {
            tauri::WindowEvent::Destroyed => {
                if window.label() != "main" {
                    return;
                }
                const CREATE_NO_WINDOW: u32 = 0x08000000;
                let mut killer = Command::new("taskkill");
                killer.args(&["/F", "/IM", "ai-brain.exe", "/T"]);
                killer.creation_flags(CREATE_NO_WINDOW);
                let _ = killer.spawn();
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_env,
            commands::write_config,
            commands::read_config,
            commands::get_log_file_location
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn strip_non_printable_chars(s: &str) -> String {
    s.chars()
        .filter(|&c| {
            // Keep printable ASCII characters (0x20 to 0x7E),
            // and common whitespace (tab, newline, carriage return)
            // and generally allow valid unicode characters.
            // Explicitly exclude null byte (0x00) and other control characters (0x01-0x1F, 0x7F)
            !matches!(c, '\x00'..='\x1F' | '\x7F') || matches!(c, '\t' | '\n' | '\r')
        })
        .collect()
}
