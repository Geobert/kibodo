use std::sync::atomic::{AtomicU64, Ordering};

use iced::{
    alignment::{Horizontal, Vertical},
    widget::canvas::{path::Builder, Frame, Text},
    Color, Point, Rectangle, Size,
};
use imstr::ImString;

use crate::klayout::ICON_FONT;

pub static KEY_ID: AtomicU64 = AtomicU64::new(1);
pub const UNIT: f32 = 50.0;

pub fn reset_key_id() {
    KEY_ID.store(1, Ordering::SeqCst);
}

// const used with KLabel’s array
const LEFT: usize = 0;
const CENTER: usize = 1;
const RIGHT: usize = 2;

#[derive(Debug, Clone, Default)]
struct KLabel {
    top: [ImString; 3],
    center: [ImString; 3],
    bottom: [ImString; 3],
}

// Rectangular shape that will deform a key
#[derive(Debug, Clone)]
pub struct Deformation {
    // top_left point, relative to thes key it belongs
    pos: Point,
    size: Size<f32>,
}

impl Deformation {
    fn abs_pos(&self, top_left: &Point) -> Point {
        Point::new(
            top_left.x + (self.pos.x * UNIT),
            top_left.y + (self.pos.y * UNIT),
        )
    }
}

impl Default for Deformation {
    fn default() -> Self {
        Self {
            pos: Default::default(),
            size: Size::new(1.0, 1.0),
        }
    }
}

#[derive(Debug)]
pub enum KeyState {
    Selected,
    Hover,
    None,
}

#[derive(Debug, Clone)]
pub struct Key {
    id: u64,
    pos: Point,
    size: Size<f32>, // exprimed in key unit (1U, 1.25U etc.)
    deform: Deformation,
    rectangle: [Rectangle; 2],
    label: KLabel,
    selected: bool,
}

impl Key {
    pub fn new(pos: Point) -> Self {
        let mut k = Self {
            id: KEY_ID.fetch_add(1, Ordering::SeqCst),
            label: KLabel::default(),
            pos,
            size: Size::new(1.0, 1.0),
            deform: Deformation::default(),
            rectangle: [Rectangle::default(); 2],
            selected: false,
        };

        k.label.center[CENTER] = ImString::from("\u{F701}");

        k.update_rect();
        k
    }

    // We don’t use Path::rectangle so we can draw both rectangles in the same Path
    fn draw_rect(top_left: Point, px_size: Size<f32>, path_builder: &mut Builder) {
        path_builder.move_to(top_left);
        path_builder.line_to(Point::new(top_left.x + px_size.width, top_left.y));
        path_builder.line_to(Point::new(
            top_left.x + px_size.width,
            top_left.y + px_size.height,
        ));
        path_builder.line_to(Point::new(top_left.x, top_left.y + px_size.height));
        path_builder.line_to(top_left);
    }

    pub fn draw(&self, path_builder: &mut Builder) {
        self.rectangle.iter().for_each(|r| {
            Key::draw_rect(
                Point::new(r.x, r.y),
                Size::new(r.width, r.height),
                path_builder,
            );
        });
    }

    fn text_pos(&self, v: Vertical, h: Horizontal) -> Point {
        let px_size = self.px_size();
        let half_h = px_size.height / 2.0;
        let half_w = px_size.width / 2.0 - 5.0;
        match (h, v) {
            (Horizontal::Left, Vertical::Top) => {
                Point::new(self.pos.x - half_w, self.pos.y - half_h)
            }
            (Horizontal::Left, Vertical::Center) => Point::new(self.pos.x - half_w, self.pos.y),
            (Horizontal::Left, Vertical::Bottom) => {
                Point::new(self.pos.x - half_w, self.pos.y + half_h)
            }
            (Horizontal::Center, Vertical::Top) => Point::new(self.pos.x, self.pos.y - half_h),
            (Horizontal::Center, Vertical::Center) => self.pos,
            (Horizontal::Center, Vertical::Bottom) => Point::new(self.pos.x, self.pos.y + half_h),
            (Horizontal::Right, Vertical::Top) => {
                Point::new(self.pos.x + half_w, self.pos.y - half_h)
            }
            (Horizontal::Right, Vertical::Center) => Point::new(self.pos.x + half_w, self.pos.y),
            (Horizontal::Right, Vertical::Bottom) => {
                Point::new(self.pos.x + half_w, self.pos.y + half_h)
            }
        }
    }

    fn draw_one_text(&self, text: &ImString, vpos: Vertical, hpos: Horizontal, frame: &mut Frame) {
        if !text.is_empty() {
            let mut text = Text::from(text.as_str());
            text.position = self.text_pos(vpos, hpos);
            text.color = Color::WHITE;
            text.vertical_alignment = vpos;
            text.horizontal_alignment = hpos;
            text.font = *ICON_FONT;
            frame.fill_text(text);
        }
    }

    pub fn draw_text(&self, frame: &mut Frame) {
        for h in 0..=2 {
            for v in [Vertical::Top, Vertical::Center, Vertical::Bottom] {
                let text = match v {
                    Vertical::Top => &self.label.top[h],
                    Vertical::Center => &self.label.center[h],
                    Vertical::Bottom => &self.label.bottom[h],
                };
                let hpos = match h {
                    0 => Horizontal::Left,
                    1 => Horizontal::Center,
                    2 => Horizontal::Right,
                    _ => unreachable!("Unknown horizontal value {h}"),
                };
                self.draw_one_text(text, v, hpos, frame)
            }
        }
    }

    fn px_size(&self) -> Size {
        Size::new(self.size.width * UNIT, self.size.height * UNIT)
    }

    #[inline]
    fn contains(&self, pos: Point) -> bool {
        self.rectangle.iter().any(|r| r.contains(pos))
    }

    pub fn state(&self, cur_pos: Option<Point>) -> KeyState {
        if self.selected {
            KeyState::Selected
        } else if let Some(cur_pos) = cur_pos {
            if self.contains(cur_pos) {
                KeyState::Hover
            } else {
                KeyState::None
            }
        } else {
            KeyState::None
        }
    }

    fn update_rect(&mut self) {
        // convert Unit size to pixel size
        let px_size = self.px_size();
        let top_left = Point::new(
            self.pos.x - (px_size.width / 2.0),
            self.pos.y - (px_size.height / 2.0),
        );
        self.rectangle[0].x = top_left.x;
        self.rectangle[0].y = top_left.y;
        self.rectangle[0].height = px_size.height;
        self.rectangle[0].width = px_size.width;

        let deform_pos = self.deform.abs_pos(&top_left);
        let deform_px_size = Size::new(
            self.deform.size.width * UNIT,
            self.deform.size.height * UNIT,
        );
        self.rectangle[1].x = deform_pos.x;
        self.rectangle[1].y = deform_pos.y;
        self.rectangle[1].height = deform_px_size.height;
        self.rectangle[1].width = deform_px_size.width;
    }
}
