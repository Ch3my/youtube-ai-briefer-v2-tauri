use futures_util::{SinkExt, StreamExt};
use tauri::{api::process::Command, Builder};
use std::process::{Command as StdCommand};

use tokio::{net::TcpStream, sync::Mutex};
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};

struct AppState {
    ws: Option<WebSocketStream<MaybeTlsStream<TcpStream>>>,
}

impl AppState {
    async fn new() -> Self {
        // let ai_brain_process = Command::new(
        //     "C:\\Github\\youtube-ai-briefer-v2\\youtube-ia-briefer-tauri\\ai-brain.exe",
        // )
        // .spawn()
        // .expect("Failed to start ai_brain.exe");

        // Need to have the correct name according to: https://tauri.app/v1/guides/building/sidecar/
        // sidecar kills on process on exit I need to kill the other
        let (mut rx, mut child) = Command::new_sidecar("ai-brain")
            .expect("failed to create `my-sidecar` binary command")
            .spawn()
            .expect("Failed to spawn sidecar");

        // You can modify the URL as per your requirements.
        let url = "ws://localhost:12345"; // Example WebSocket URL

        // Create a default WebSocket connection (asynchronous operation)
        let (ws, _) = connect_async(url)
            .await
            .expect("Failed to connect to WebSocket");

        // Print a success message when the connection is established
        println!("Successfully connected to WebSocket at {}", url);

        AppState {
            ws: Some(ws),
        }
    }
    async fn close_ws(&mut self) {
        if let Some(mut ws) = self.ws.take() {
            // Close the WebSocket gracefully
            let _ = ws.close(None).await;
            println!("WebSocket connection closed.");
        }
    }

    fn kill_ai_brain(&mut self) {
        #[cfg(target_os = "windows")]
        {
            let output = StdCommand::new("taskkill")
                .arg("/F")
                .arg("/IM")
                .arg("ai-brain.exe")
                .output()
                .expect("Failed to execute taskkill command");

            // Log output
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            println!("Output: {}", stdout);
            if !stderr.is_empty() {
                println!("Error: {}", stderr);
            }
        }
    }
}

#[tauri::command]
async fn send(state: tauri::State<'_, Mutex<AppState>>, message: String) -> Result<(), String> {
    let mut state = state.lock().await;

    if let Some(ws) = &mut state.ws {
        // Attempt to send the message
        ws.send(Message::Text(message))
            .await
            .map_err(|e| format!("Failed to send message: {}", e))?;

        // No need to wait for a response
        Ok(())
    } else {
        Err("WebSocket connection is not established".into())
    }
}

#[tauri::command]
async fn on_exit(state: tauri::State<'_, Mutex<AppState>>) -> Result<String, String> {
    let mut state = state.lock().await;
    state.kill_ai_brain();
    state.close_ws().await;
    std::process::exit(0);
}

#[tokio::main]
async fn main() {
    // Create the AppState asynchronously
    let app_state = Mutex::new(AppState::new().await);

    Builder::default()
        .manage(app_state)
        // .on_window_event(|event| {
        //     match event.event() {
        //         tauri::WindowEvent::CloseRequested { api, .. } => {
        //         }
        //         _ => {}
        //     }
        // })
        .invoke_handler(tauri::generate_handler![send, on_exit])
        .run(tauri::generate_context!())
        .unwrap();
}
