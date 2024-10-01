// Avoid terminal on exe on production
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{
    api::process::{Command, CommandChild, CommandEvent},
    Builder, Manager,
};
use tokio::sync::Mutex;

struct AppState {
    ai_brain: Option<CommandChild>,
}

impl Default for AppState {
    fn default() -> Self {
        let (mut rx, child) = Command::new_sidecar("ai-brain")
            .expect("failed to create `my-sidecar` binary command")
            .spawn()
            .expect("Failed to spawn sidecar");

        // To show SideCar logs in same window as Tauri
        tauri::async_runtime::spawn(async move {
            while let Some(event) = rx.recv().await {
                match event {
                    CommandEvent::Stdout(line) => {
                        println!("AI Brain stdout: {}", line);
                    }
                    CommandEvent::Stderr(line) => {
                        eprintln!("AI Brain stderr: {}", line);
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
        // if let Some(child) = self.ai_brain.take() {
        //     let _ = Command::new("taskkill")
        //         .args(&["/F", "/T", "/PID", &child.pid().to_string()])
        //         .output();
        // }
        // Kill all instances of ai-brain.exe
        let _ = Command::new("taskkill")
            .args(&["/F", "/IM", "ai-brain.exe", "/T"])
            .output();
    }
}

#[tauri::command]
fn get_env(name: &str) -> String {
    std::env::var(String::from(name)).unwrap_or(String::from(""))
}

fn main() {
    Builder::default()
        .manage(Mutex::new(AppState::default()))
        .on_window_event(|event| {
            match event.event() {
                tauri::WindowEvent::Destroyed => {
                    if event.window().label() != "main" {
                        // Not main window, maybe splashScreen
                        return;
                    }
                    let state: tauri::State<'_, Mutex<AppState>> = event.window().state();
                    let rt = tokio::runtime::Runtime::new().unwrap();
                    let mut sst = rt.block_on(state.lock());
                    sst.cleanup();
                }
                _ => {}
            }
        })
        .invoke_handler(tauri::generate_handler![get_env])
        .run(tauri::generate_context!())
        .unwrap();
}
