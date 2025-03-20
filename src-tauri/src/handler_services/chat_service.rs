use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use tauri::Emitter;

use crate::app_state::Client;
use crate::proto_types::message;

use super::StateAccessor;

#[derive(Clone, Serialize, Deserialize)]
pub struct MessageWrapper {
    pub user_id: String,
    pub content: String,
    pub sent_date: Option<i64>,
}

impl From<message::Message> for MessageWrapper {
    fn from(msg: message::Message) -> Self {
        Self {
            user_id: msg.user_id,
            content: msg.content,
            sent_date: msg.sent_date.map(|ts| ts.seconds),
        }
    }
}

pub struct ChatClient {
    app_handle: tauri::AppHandle,
}

impl ChatClient {
    pub fn new(app_handle: &tauri::AppHandle) -> Self {
        Self { app_handle: app_handle.clone() }
    }

    pub fn establish_connection(&self) {
        let handle = self.app_handle.clone();
        tauri::async_runtime::spawn(async move {
            loop {
                let client_option = handle.with_state(|state| state.client.clone());

                if client_option.is_none() {
                    match Client::connect("http://localhost:5197").await {
                        Ok(client) => {
                            {
                                println!("Connected!");
                                handle.with_state_mut(|state| {
                                    state.client = Some(client.clone());
                                    state.connected = true;
                                })
                            }

                            Self::handle_stream(handle.clone(), client);
                        }
                        Err(e) => {
                            eprintln!("Connection error: {}", e)
                        }
                    }
                }

                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            }
        });
    }

    pub fn handle_stream(app_handle: tauri::AppHandle, mut client: Client) {
        tauri::async_runtime::spawn(async move {
            let user = app_handle.with_state(|state| state.get_user());
            let request = tonic::Request::new(message::Connect {
                user: Some(user),
                active: true,
            });

            match client.connect_request(request).await {
                Ok(response) => {
                    let mut stream = response.into_inner();

                    while let Some(result) = stream.next().await {
                        match result {
                            Ok(msg) => {
                                println!("{:?}", msg);
                                let wrapper = MessageWrapper::from(msg);
                                app_handle
                                    .emit("new_message", wrapper)
                                    .expect("Failed to emit message");
                            }
                            Err(e) => {
                                eprintln!("Stream error: {}", e);
                                break;
                            }
                        }
                    }

                    app_handle.with_state_mut(|state| {
                        state.connected = false;
                    });
                }
                Err(e) => {
                    eprint!("Error with connection: {}", e);

                    app_handle.with_state_mut(|state| {
                        state.connected = false;
                    });
                }
            }
        });
    }
}