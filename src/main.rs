use iced::widget::{button, column, text, Column};

fn main() -> iced::Result {
    iced::run("Mice", Counter::update, Counter::view)
}

#[derive(Clone, Copy, Debug)]
enum Message {
    Increment,
    Decrement
}

#[derive(Default)]
struct Counter {
    value: i64
}

impl Counter {
    pub fn view(&self) -> Column<Message> {
        column![
            button("+").on_press(Message::Increment),
            text(self.value).size(50),
            button("-").on_press(Message::Decrement),
        ]
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Decrement => self.value -= 1,
            Message::Increment => self.value += 1
        }
    }
}
