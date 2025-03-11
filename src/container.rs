use std::sync::Arc;

use tokio::sync::Mutex;

use crate::client::{GrcpMessageService, MessageService};

struct AppContainer {
    message_service: Arc<Mutex<dyn MessageService>>,
    // ...
}

impl AppContainer {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error + Send + Sync >> {
        let message_service = GrcpMessageService::new("http://localhost:5197".to_string()).await?;

        Ok(Self {
            message_service: Arc::new(Mutex::new(message_service)),
        })
    }
}