use macroquad::prelude::*;

use crate::game::{rendering::Drawable, BASE_WIDTH};

/// A single line of static heading text, e.g. a page's title screen heading.
pub struct Title {
    text: String,
    font_size: u16,
    position: Vec2,
    color: Color,
    font: Option<Font>,
}

impl Title {
    /// `font` renders the title with a loaded custom font instead of
    /// macroquad's built-in default; pass `None` to use the default.
    pub fn new(text: String, font_size: u16, position: Vec2, color: Color, font: Option<Font>) -> Self {
        Self { text, font_size, position, color, font }
    }
}

impl Drawable for Title {
    fn draw(&self) {
        let scale = screen_width() / BASE_WIDTH;
        let font_size = (self.font_size as f32 * scale).round().max(8.0) as u16;

        draw_text_ex(
            &self.text,
            self.position.x,
            self.position.y,
            TextParams {
                font: self.font.as_ref(),
                font_size,
                color: self.color,
                ..Default::default()
            },
        );
    }
}
