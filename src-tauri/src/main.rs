// Avoid terminal on exe on production
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod app_state;
use commands::{get_env, write_config, cleanup, read_config};
use app_state::AppState;

use tokio::sync::Mutex;
use tauri::{Builder, Manager};

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
                    // let state: tauri::State<'_, Mutex<AppState>> = event.window().state();
                    // let rt = tokio::runtime::Runtime::new().unwrap();
                    // let mut sst = rt.block_on(state.lock());
                    // sst.cleanup();
                    cleanup(&event.window().state());
                }
                _ => {}
            }
        })
        .invoke_handler(tauri::generate_handler![get_env, write_config, read_config])
        .run(tauri::generate_context!())
        .unwrap();
}
