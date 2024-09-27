// websocket_handler.rs

use tauri::{Manager, State};
use tokio::sync::Mutex;

// Define your AppState structure
#[derive(Default)]
pub struct AppState {
    pub counter: u32,
}

#[tauri::command]
pub async fn send(state: State<'_, Mutex<AppState>>) -> Result<u32, ()> {
  let mut state = state.lock().await;
  state.counter += 1;
  Ok(state.counter)
}

// Function to setup the AppState
pub fn setup(app: &mut tauri::App) -> tauri::Result<()> {
        // Initialize the AppState and attach it to the app
    app.manage(AppState::default());
    Ok(())
}
