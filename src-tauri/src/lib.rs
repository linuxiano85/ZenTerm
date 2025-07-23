use std::sync::Mutex;
use tauri::State;

mod terminal;
use terminal::{SessionManager, TerminalSession};

// Global state for the session manager
struct AppState {
    session_manager: Mutex<SessionManager>,
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn create_terminal_session(
    name: String,
    state: State<AppState>,
) -> Result<String, String> {
    state
        .session_manager
        .lock()
        .unwrap()
        .create_session(name)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn execute_command(
    session_id: String,
    command: String,
    state: State<AppState>,
) -> Result<String, String> {
    state
        .session_manager
        .lock()
        .unwrap()
        .execute_command(&session_id, &command)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn get_sessions(state: State<AppState>) -> Vec<TerminalSession> {
    state.session_manager.lock().unwrap().list_sessions()
}

#[tauri::command]
fn set_active_session(
    session_id: String,
    state: State<AppState>,
) -> Result<(), String> {
    state
        .session_manager
        .lock()
        .unwrap()
        .set_active_session(session_id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn resize_terminal(
    session_id: String,
    rows: u16,
    cols: u16,
    state: State<AppState>,
) -> Result<(), String> {
    state
        .session_manager
        .lock()
        .unwrap()
        .resize_session(&session_id, rows, cols)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn close_session(
    session_id: String,
    state: State<AppState>,
) -> Result<(), String> {
    state
        .session_manager
        .lock()
        .unwrap()
        .close_session(&session_id)
        .map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState {
            session_manager: Mutex::new(SessionManager::new()),
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            create_terminal_session,
            execute_command,
            get_sessions,
            set_active_session,
            resize_terminal,
            close_session
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
