// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod commands;
use std::env;
use std::process::Command;
use tauri_plugin_shell::process::CommandEvent;
use tauri_plugin_shell::ShellExt;
use std::os::windows::process::CommandExt;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                let sidecar_command = handle.shell().sidecar("ai-brain").unwrap();
                let (mut rx, _child) = sidecar_command.spawn().unwrap();

                while let Some(event) = rx.recv().await {
                    match event {
                        CommandEvent::Stdout(line_bytes) => {
                          let line = String::from_utf8_lossy(&line_bytes);
                          println!("AI Brain stdout: {}", line);
                        }
                        CommandEvent::Stderr(line_bytes) => {
                          let line = String::from_utf8_lossy(&line_bytes);
                          println!("AI Brain stdout: {}", line);
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
