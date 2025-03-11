use std::sync::Arc;

use client::{message, MessageService};
use iced::widget::{button, column, row, text, text_input, Column};
use tokio::sync::Mutex;

mod client;
mod container;

fn main() -> iced::Result {
    iced::application("Mice", App::update, App::view)
        .run()
}

#[derive(Clone, Debug)]
enum Message {}

#[derive(Default)]
struct App {
    message_service: Option<Arc<Mutex<dyn MessageService>>>,
    input_value: String,
    message_list: Vec<message::Message>,
}

impl App {
    pub fn view(&self) -> Column<Message> {
        column![

        ]
    }

    pub fn update(&mut self, message: Message) {}
}
