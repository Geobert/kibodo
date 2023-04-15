use iced::{
    widget::{button, column, text},
    Alignment, Element, Length, Point, Sandbox, Settings,
};
use key::reset_key_id;

mod key;
mod klayout;

fn main() -> iced::Result {
    Kibodo::run(Settings::default())
}

#[derive(Default)]
struct Kibodo {
    layout: klayout::KLayout,
}

#[derive(Debug, Clone)]
pub enum Message {
    AddKey(key::Key),
    Redraw(Point),
    Clear,
}

impl Sandbox for Kibodo {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Kibodo")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::AddKey(key) => {
                self.layout.add_key(key);
            }
            Message::Redraw(_pos) => (),
            Message::Clear => {
                self.layout.clear();
                reset_key_id();
            }
        }
        self.layout.request_redraw();
    }

    fn view(&self) -> Element<'_, Self::Message> {
        column![
            text("Kibodo").width(Length::Shrink).size(50),
            self.layout.view(),
            button("Clear").padding(8).on_press(Message::Clear)
        ]
        .padding(20)
        .spacing(20)
        .align_items(Alignment::Center)
        .into()
    }
}
