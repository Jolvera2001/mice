use std::sync::Mutex;
use chrono::{self, Utc};
use tauri::State;

use crate::{app_state::{AppState, MessageWrapper}, proto_types::message::{self}};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
pub async fn send_message(state: State<'_, Mutex<AppState>>, msg: &str) -> Result<MessageWrapper, String> {
    let user_id;
    let client_clone;
    
    // to get around mutex guard issue
    {
        let state_guard = state.lock().unwrap();
        client_clone = state_guard.client.clone();
        user_id = state_guard.get_user().id;
    }
    
    let curr_date = Some(prost_types::Timestamp { seconds: Utc::now().timestamp(), nanos: 0 });

    if let Some(mut client) = client_clone {
        let request = message::Message {
            user_id: user_id.clone(),
            content: msg.to_string(),
            sent_date: curr_date
        };

        let _ = client.broadcast_message(tonic::Request::new(request))
            .await
            .map_err(|e| e.to_string())?;

        println!("content: {}", msg.to_string());

        Ok(MessageWrapper {
            user_id,
            content: msg.to_string(),
            sent_date: curr_date.map(|ts| ts.seconds)
        })
    } else {
        return Err("Not connected to the server".to_string())
    }
}
