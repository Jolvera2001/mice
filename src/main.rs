use iced::widget::{button, column, row, text, text_input, Column};

fn main() -> iced::Result {
    iced::run("Mice", App::update, App::view)
}

#[derive(Clone, Debug)]
enum Message {
    Add,
    Delete(usize),
    ContentChanged(String)
}

#[derive(Default)]
struct App {
    list: Vec<String>,
    content: String,
    warning: String,
}

impl App {
    pub fn view(&self) -> Column<Message> {
        let mut todo_items = Vec::new();

        for (index, item) in self.list.iter().enumerate() {
            todo_items.push(
                row![
                    button("-").on_press(Message::Delete(index)),
                    text(item).size(16),
                ]
                .spacing(6)
                .into()
            );
        }

        let input_row = row![
            text_input("Type here...", &self.content)
                .on_input(Message::ContentChanged),
            button("Add").on_press(Message::Add)
        ].spacing(10);

        let warning_text = if !self.warning.is_empty() {
            text(&self.warning)
        } else {
            text("")
        };

        column![
            column(todo_items).spacing(5),
            input_row,
            warning_text
        ]
        .spacing(20)
        .padding(20)
        .into()

    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::ContentChanged(content) => {
                self.content = content
            },
            Message::Add =>  {
                if !self.content.is_empty() && !self.content.trim().is_empty() {
                    self.warning.clear();
                    self.list.push(self.content.clone());
                    self.content.clear();
                } else {
                    self.warning = String::from("Cannot add an empty task");
                }
            },
            Message::Delete(location) => {
                self.list.remove(location);
            }
        }
    }
}