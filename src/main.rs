use std::sync::Arc;

use client::{message, MessageService};
use container::AppContainer;
use iced::{
    widget::{button, column, row, text, text_input, Column},
    Task,
};
use tokio::process::Command;

mod client;
mod container;

fn main() -> iced::Result {
    let runtime = tokio::runtime::Runtime::new().expect("Failed to create tokio runtime");

    let app = runtime.block_on(async { App::new().await });

    iced::application("Mice", App::update, App::view)
        // .subscription()
        .run()
}

#[derive(Clone, Debug)]
enum AppMessage {
    MessageSent,
    MessageSentSuccess,
    MessageSentError(String),
    TextBoxChanged(String),
    MessageRecieved(message::Message),
}

#[derive(Default)]
struct App {
    container: AppContainer,
    input_value: String,
    message_list: Vec<message::Message>,
}

impl App {
    pub async fn new() -> Self {
        App {
            container: container::AppContainer::new()
                .await
                .expect("Failed to initialize container"),
            input_value: String::from(""),
            message_list: Vec::new(),
        }
    }

    pub fn view(&self) -> Column<AppMessage> {
        let messages = self.message_list
        .iter()
        .map(| item| {
            row![
                text(&item.content).size(16)
            ]
            .spacing(6)
            .into()
        }).collect::<Vec<_>>();

        column![

            row![
                text_input("Type here...", &self.input_value)
                    .on_input(AppMessage::TextBoxChanged),
                button("Send").on_press(AppMessage::MessageSent)
            ].spacing(10),
            column(messages).spacing(10)
        ]
    }

    pub fn update(&mut self, message: AppMessage) -> Task<AppMessage> {
        match message {
            AppMessage::MessageRecieved(content) => {
                self.message_list.push(content);
                Task::none()
            }
            AppMessage::MessageSent => {
                let service = self.container.message_service();
                let content = self.input_value.clone();
                self.input_value.clear();

                Task::perform(
                    async move {
                        let mut client = service.lock().await;
                        client.send_message(content).await
                    },
                    |result| match result {
                        Ok(_) => AppMessage::MessageSentSuccess,
                        Err(e) => AppMessage::MessageSentError(e.to_string()),
                    },
                )
            }
            AppMessage::TextBoxChanged(content) => {
                self.input_value = content;
                Task::none()
            },
            AppMessage::MessageSentSuccess => {
                Task::none()
            },
            AppMessage::MessageSentError(e) => {
                Task::none()
            }
        }
    }
}
