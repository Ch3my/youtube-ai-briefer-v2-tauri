// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod commands;
use chrono::Local;
use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::os::windows::process::CommandExt;
use std::path::PathBuf;
use std::process::Command;
use tauri_plugin_shell::process::CommandEvent;
use tauri_plugin_shell::ShellExt; // Import PathBuf for constructing file paths

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                let log_file_path = match env::current_exe() {
                    Ok(mut path) => {
                        path.pop(); // Remove the executable name
                        path.push("log.txt"); // Add the log file name
                        path
                    }
                    Err(_) => {
                        // Fallback to current directory if executable path can't be determined
                        PathBuf::from("log.txt")
                    }
                };

                // Open the log file in append mode, create if it doesn't exist
                let mut log_file = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(&log_file_path)
                    .expect("Failed to open log file");

                let sidecar_command = handle.shell().sidecar("ai-brain").unwrap();
                let (mut rx, _child) = sidecar_command.spawn().unwrap();

                while let Some(event) = rx.recv().await {
                    // Get current local datetime
                    let now = Local::now();
                    let timestamp = now.format("%Y-%m-%d %H:%M:%S").to_string(); // Format as YYYY-MM-DD HH:MM:SS

                    match event {
                        CommandEvent::Stdout(line_bytes) => {
                            let line = String::from_utf8_lossy(&line_bytes);
                            let cleaned_line = strip_non_printable_chars(&line); // Clean the line
                            println!("[{}] AI Brain stdout: {}", timestamp, line);
                            if let Err(e) = writeln!(
                                log_file,
                                "[{}] AI Brain stdout: {}",
                                timestamp, cleaned_line
                            ) {
                                eprintln!("Failed to write to log file: {}", e);
                            }
                        }
                        CommandEvent::Stderr(line_bytes) => {
                            let line = String::from_utf8_lossy(&line_bytes);
                            let cleaned_line = strip_non_printable_chars(&line); // Clean the line, remove terminal colors
                            println!("[{}] AI Brain stderr: {}", timestamp, line);
                            if let Err(e) = writeln!(
                                log_file,
                                "[{}] AI Brain stderr: {}",
                                timestamp, cleaned_line
                            ) {
                                eprintln!("Failed to write to log file: {}", e);
                            }
                        }
                        _ => {}
                    }
                }
            });

            Ok(())
        })
        .on_window_event(|window, event| {
            match event {
                tauri::WindowEvent::Destroyed => {
                    if window.label() != "main" {
                        // Not main window, maybe splashScreen
                        return;
                    }
                    const CREATE_NO_WINDOW: u32 = 0x08000000;
                    let mut killer = Command::new("taskkill");
                    killer.args(&["/F", "/IM", "ai-brain.exe", "/T"]);
                    // This FLAG works only windows, needed on other OS?
                    killer.creation_flags(CREATE_NO_WINDOW);
                    let _ = killer.spawn();
                }
                _ => {}
            }
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_env,
            commands::write_config,
            commands::read_config
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
