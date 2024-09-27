use futures_util::{SinkExt, StreamExt};
use std::process::{Child, Command};
use tokio::{net::TcpStream, sync::Mutex};
use tauri::{Builder, Manager};
use tokio_tungstenite::{connect_async, tungstenite::{self, Message}, MaybeTlsStream, WebSocketStream};

struct AppState {
    ws: Option<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    ai_brain_process: Option<Child>,
}

impl AppState {
    async fn new() -> Self {
        let ai_brain_process = Command::new(
            "C:\\Github\\youtube-ai-briefer-v2\\youtube-ia-briefer-tauri\\ai-brain.exe",
        )
        .spawn()
        .expect("Failed to start ai_brain.exe");

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
            ai_brain_process: Some(ai_brain_process),
        }
    }
}

#[tauri::command]
async fn send(state: tauri::State<'_, Mutex<AppState>>, message: String) -> Result<String, String> {
    let mut state = state.lock().await;

    if let Some(ws) = &mut state.ws {
        // Attempt to send the message
        ws.send(Message::Text(message)).await.map_err(|e| {
            format!("Failed to send message: {}", e)
        })?;

        // Wait for a response from the WebSocket
        if let Some(response) = ws.next().await {
            match response {
                Ok(Message::Text(text)) => Ok(text), // Return the response as a string
                Ok(Message::Close(_)) => Err("WebSocket connection closed".into()),
                Err(e) => Err(format!("Error receiving response: {}", e)),
                _ => Err("Received an unexpected message type".into()),
            }
        } else {
            Err("No response received".into())
        }
    } else {
        Err("WebSocket connection is not established".into())
    }
}


#[tokio::main]
async fn main() {
    // Create the AppState asynchronously
    let app_state = Mutex::new(AppState::new().await);

    Builder::default()
        .setup(|app| {
            app.manage(app_state);
            Ok(())
        })
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { api, .. } => {
            //   event.window().hide().unwrap();
            //   api.prevent_close();
            print!("Window Closing");
            }
            _ => {}
          })
        .invoke_handler(tauri::generate_handler![send])
        .run(tauri::generate_context!())
        .unwrap();
}
