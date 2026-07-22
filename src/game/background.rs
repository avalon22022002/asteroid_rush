use macroquad::{color::BLACK, window::clear_background};

use crate::game::{background::starfield::*, rendering::{Drawable, StateUpdatable}};

pub mod starfield;

pub struct Background{
    star_field: Starfield
}

impl Background{
    pub fn new()-> Self {
        Background{
            star_field: Starfield::new(150, None)
        }
    }
}

impl Drawable for Background{
    fn draw(&self) {
        clear_background(BLACK);
        self.star_field.draw();
    }
}

impl StateUpdatable<()> for Background{
    fn update_state(&mut self, data: ()) {
        // update starfield's state
        self.star_field.update_state(data);
    }
}