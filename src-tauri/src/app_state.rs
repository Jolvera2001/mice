use message::message_service_client::MessageServiceClient;
use serde::{Deserialize, Serialize};
use tonic::transport::Channel;

use crate::proto_types::message;

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

pub type Identifier = message::User;
pub type Client = MessageServiceClient<Channel>;

pub struct AppState {
    pub user: Identifier,
    pub client: Option<Client>,
    pub connected: bool,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            user: Identifier {
                id: uuid::Uuid::new_v4().to_string(),
                name: "test".to_string(),
            },
            client: None,
            connected: false,
        }
    }
    pub fn get_user(&self) -> Identifier {
        self.user.clone()
    }
}