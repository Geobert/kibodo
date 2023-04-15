use iced::{
    mouse,
    widget::canvas::{
        self,
        event::{self, Event},
        Canvas, Cursor, Fill, FillRule, Frame, Geometry, Path, Stroke,
    },
    Color, Element, Font, Length, Point, Rectangle, Theme,
};
use once_cell::sync::Lazy;

use crate::{
    key::{Key, KeyState},
    Message,
};

pub static ICON_FONT: Lazy<Font> = Lazy::new(|| {
    // let mut tape = std::fs::File::open("./bootstrap-icons.woff2")
    //     .expect("Error opening `./bootstrap-icons.woff2`");
    // let webtype::File { fonts: _, tape } =
    //     webtype::File::read(&mut tape).expect("Error reading font");
    // let font = Box::new(tape.into_inner());

    let font = Box::new(std::fs::read("./bootstrap-icons.ttf").expect("Error reading font"));
    iced_native::Font::External {
        name: "Icons",
        bytes: font.leak(),
    }
});

#[derive(Default)]
pub struct KLayout {
    cache: canvas::Cache,
    keys: Vec<Key>,
}

impl KLayout {
    pub(crate) fn clear(&mut self) {
        self.keys.clear();
    }

    pub(crate) fn add_key(&mut self, key: Key) {
        self.keys.push(key);
    }

    pub(crate) fn request_redraw(&mut self) {
        self.cache.clear()
    }

    pub fn view(&self) -> Element<Message> {
        Canvas::new(self)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    pub fn draw_all(&self, frame: &mut Frame, cursor_pos: Option<Point>) {
        for key in &self.keys {
            // draw key frames
            let path = Path::new(|builder| key.draw(builder));

            // key border color according to state
            let stroke = match key.state(cursor_pos) {
                KeyState::Selected => Stroke::default().with_color(Color::from_rgb8(255, 0, 0)),
                KeyState::Hover => Stroke::default().with_color(Color::from_rgb8(0, 255, 0)),
                KeyState::None => Stroke::default(),
            };

            // apply drawn path
            frame.stroke(&path, stroke.with_width(2.0));

            // Fill keys with a color (so we mask the inner borders)
            frame.fill(
                &path,
                Fill {
                    style: canvas::Style::Solid(Color::from_rgb8(111, 111, 186)),
                    rule: FillRule::NonZero,
                },
            );

            // Draw text
            key.draw_text(frame);
        }
    }
}

impl canvas::Program<Message> for KLayout {
    type State = ();

    fn update(
        &self,
        _state: &mut Self::State,
        event: Event,
        bounds: Rectangle,
        cursor: Cursor,
    ) -> (event::Status, Option<Message>) {
        let cur_pos = if let Some(pos) = cursor.position_in(&bounds) {
            pos
        } else {
            return (event::Status::Ignored, None);
        };

        match event {
            Event::Mouse(m_evt) => {
                let msg = match m_evt {
                    mouse::Event::ButtonPressed(mouse::Button::Left) => {
                        Some(Message::AddKey(Key::new(cur_pos)))
                    }
                    mouse::Event::CursorMoved { position } => Some(Message::Redraw(position)),
                    _ => None,
                };
                (event::Status::Captured, msg)
            }
            _ => (event::Status::Ignored, None),
        }
    }

    fn draw(
        &self,
        _state: &Self::State,
        _theme: &Theme,
        bounds: Rectangle,
        cursor: Cursor,
    ) -> Vec<Geometry> {
        let content = self.cache.draw(bounds.size(), |frame: &mut Frame| {
            // draw the keys
            self.draw_all(frame, cursor.position_in(&bounds));

            // frame borders
            frame.stroke(
                &Path::rectangle(Point::ORIGIN, frame.size()),
                Stroke::default().with_width(2.0),
            );
        });
        vec![content]
    }

    // fn mouse_interaction(
    //     &self,
    //     _state: &Self::State,
    //     bounds: Rectangle,
    //     cursor: Cursor,
    // ) -> mouse::Interaction {
    //     if cursor.is_over(&bounds) {
    //         mouse::Interaction::Crosshair
    //     } else {
    //         mouse::Interaction::default()
    //     }
    // }
}
