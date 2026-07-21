use macroquad::{color::BLACK, window::clear_background};

use crate::game::rendering::Drawable;

pub mod stars;

pub struct Background{
    star_field: stars::Starfield
}

impl Background{
    pub fn new()-> Self {
        Background{
            star_field: stars::generate_starfield(150, None)
        }
    }
}

impl Drawable for Background{
    fn draw(&self) {
        clear_background(BLACK);
        self.star_field.draw();
    }
}
