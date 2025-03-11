use std::sync::Arc;

use tokio::sync::Mutex;

use crate::client::{GrcpMessageService, MessageService};

pub struct AppContainer {
    message_service: Arc<Mutex<dyn MessageService + Send + Sync>>,
    // ...
}

impl AppContainer {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let message_service = GrcpMessageService::new("http://localhost:5197".to_string()).await?;

        Ok(Self {
            message_service: Arc::new(Mutex::new(message_service)),
        })
    }

    pub fn message_service(&self) -> Arc<Mutex<dyn MessageService + Send + Sync>> {
        Arc::clone(&self.message_service)
    }
}

impl Default for AppContainer {
    fn default() -> Self {
        AppContainer {
            message_service: Arc::new(Mutex::new(GrcpMessageService::default())),
        }
    }
}
