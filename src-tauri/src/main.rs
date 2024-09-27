// Avoid terminal on exe on production
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tokio::sync::Mutex;
use tauri::{
    api::process::{Command, CommandChild},
    Builder, Manager,
};

struct AppState {
    ai_brain: Option<CommandChild>,
}

impl Default for AppState {
    fn default() -> Self {
        let (_rx, child) = Command::new_sidecar("ai-brain")
            .expect("failed to create `my-sidecar` binary command")
            .spawn()
            .expect("Failed to spawn sidecar");

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
    }
}

fn main() {
    Builder::default()
        .manage(Mutex::new(AppState::default()))
        .on_window_event(|event| {
            match event.event() {
                tauri::WindowEvent::CloseRequested { .. } => {
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
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .unwrap();
}
