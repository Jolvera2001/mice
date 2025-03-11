use std::error::Error;
use chrono::{DateTime, Utc};
use message::message_service_client::MessageServiceClient;
use prost_types::Timestamp;
use message::Message;
use tonic::{async_trait, transport::Channel};
use uuid::Uuid;

pub mod message {
    tonic::include_proto!("message");
}

#[async_trait]
pub trait MessageService {
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

#[async_trait]
impl MessageService for GrcpMessageService {
    async fn send_message(&mut self, content: String) -> Result<(), Box<dyn Error + Send + Sync>> {
        let now = Utc::now();
        let timestamp = Timestamp {
            seconds: now.timestamp(),
            nanos: now.timestamp_subsec_nanos() as i32
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


// struct MessageClient;

// impl MessageClient {
//     async fn start() -> Result<(), Box<dyn std::error::Error>> {
//         let mut client = MessageServiceClient::connect("http://localhost:5197").await?;
//         let id = uuid::Uuid::new_v4();
//         let now = Utc::now();

//         let time = Timestamp {
//             seconds: now.timestamp(),
//             nanos: now.timestamp_subsec_nanos() as i32
//         };
        
//         let request = tonic::Request::new( Message {
//             user_id: id.to_string(),
//             content: "Hello world".to_string(),
//             sent_date: Some(time),
//         });

//         let mut response = client.send_message(request).await?;

//         while let Some(message) = response.get_mut().message().await? {
//             let received_time: DateTime<Utc> = if let Some(ts) = &message.sent_date {
//                 DateTime::<Utc>::from_timestamp(ts.seconds, ts.nanos as u32)
//                     .unwrap_or_else(|| Utc::now())
//             } else {
//                 Utc::now()
//             };
//         }

//         Ok(())
//     }
// }