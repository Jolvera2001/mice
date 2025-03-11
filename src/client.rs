use chrono::Utc;
use message::message_service_client::MessageServiceClient;
use message::Message;
use prost_types::Timestamp;
use std::error::Error;
use tonic::{async_trait, transport::Channel};
use uuid::Uuid;

pub mod message {
    tonic::include_proto!("message");
}

#[async_trait]
pub trait MessageService: Send + Sync {
    async fn send_message(&mut self, content: String) -> Result<(), Box<dyn Error + Send + Sync>>;
    async fn get_messages(&mut self) -> Result<Vec<Message>, Box<dyn Error + Send + Sync>>;
}

pub struct GrcpMessageService {
    client: MessageServiceClient<Channel>,
    user_id: String,
}

impl GrcpMessageService {
    pub async fn new(server_url: String) -> Result<Self, Box<dyn Error + Send + Sync>> {
        let client = MessageServiceClient::connect(server_url).await?;
        let user_id = Uuid::new_v4().to_string();

        Ok(Self {
            client: client,
            user_id: user_id,
        })
    }
}

impl Default for GrcpMessageService {
    fn default() -> Self {
        let channel = Channel::from_static("http://localhost:5197").connect_lazy();
        GrcpMessageService {
            client: MessageServiceClient::new(channel),
            user_id: 1.to_string(),
        }
    }
}

#[async_trait]
impl MessageService for GrcpMessageService {
    async fn send_message(&mut self, content: String) -> Result<(), Box<dyn Error + Send + Sync>> {
        let now = Utc::now();
        let timestamp = Timestamp {
            seconds: now.timestamp(),
            nanos: now.timestamp_subsec_nanos() as i32,
        };

        let request = tonic::Request::new(Message {
            user_id: self.user_id.clone(),
            content,
            sent_date: Some(timestamp),
        });

        self.client.send_message(request).await?;
        Ok(())
    }

    async fn get_messages(&mut self) -> Result<Vec<Message>, Box<dyn Error + Send + Sync>> {
        Ok(Vec::new())
    }
}
