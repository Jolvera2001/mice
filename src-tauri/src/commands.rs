use std::sync::Mutex;
use chrono::{self, Utc};
use tauri::State;

use crate::{app_state::AppState, proto_types::message};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
pub async fn send_message(state: State<'_, Mutex<AppState>>, msg: &str) -> Result<(), String> {
    let state_guard = state.lock().unwrap();
    if let Some(mut client) = state_guard.client.clone() {
        let request = message::Message {
            user_id: state_guard.get_user().id,
            content: msg.to_string(),
            sent_date: Some(prost_types::Timestamp { seconds: Utc::now().timestamp(), nanos: 0 })
        };

        let _ = client.broadcast_message(tonic::Request::new(request))
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    } else {
        return Err("Not connected to the server".to_string())
    }
}
