use std::sync::Mutex;
use tauri::Manager;

mod app_state;
mod handler_services;
mod proto_types;
mod commands;

use app_state::AppState;
use commands::greet;
use crate::handler_services::ChatClient;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            app.manage(Mutex::new(AppState::new()));

            let app_handle = app.handle();
            let chat_client = ChatClient::new(app_handle);

            chat_client.establish_connection();

            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
