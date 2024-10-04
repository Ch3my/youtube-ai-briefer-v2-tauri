use tauri::api::process::{Command, CommandChild, CommandEvent};
use std::fs::OpenOptions;
use std::io::Write; 

pub struct AppState {
    pub ai_brain: Option<CommandChild>,
}

impl Default for AppState {
    fn default() -> Self {
        let (mut rx, child) = Command::new_sidecar("ai-brain")
            .expect("failed to create `my-sidecar` binary command")
            .spawn()
            .expect("Failed to spawn sidecar");

        // Open the log file for appending
        let mut log_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("log.txt")
            .expect("Unable to open log file");

        // To show SideCar logs in same window as Tauri
        tauri::async_runtime::spawn(async move {
            while let Some(event) = rx.recv().await {
                match event {
                    CommandEvent::Stdout(line) => {
                        println!("AI Brain stdout: {}", line);
                        if let Err(e) = writeln!(log_file, "AI Brain stdout: {}", line) {
                            eprintln!("Failed to write to log file: {}", e);
                        }
                    }
                    CommandEvent::Stderr(line) => {
                        eprintln!("AI Brain stderr: {}", line);
                        if let Err(e) = writeln!(log_file, "AI Brain stderr: {}", line) {
                            eprintln!("Failed to write to log file: {}", e);
                        }
                    }
                    _ => {}
                }
            }
        });

        AppState {
            ai_brain: Some(child),
        }
    }
}

impl AppState {
    pub fn cleanup(&mut self) {
        // TODO. Manage other SO this is for Windows
        // Kill process and related Processes
        if let Some(child) = self.ai_brain.take() {
            let _ = Command::new("taskkill")
                .args(&["/F", "/T", "/PID", &child.pid().to_string()])
                .output();
        }
        // Kill all instances of ai-brain.exe
        let _ = Command::new("taskkill")
            .args(&["/F", "/IM", "ai-brain.exe", "/T"])
            .output();
    }
}
