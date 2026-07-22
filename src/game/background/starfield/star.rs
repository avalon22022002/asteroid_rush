use macroquad::prelude::*;

use crate::game::rendering::{Drawable, StateUpdatable};

pub struct Star {
   pub x: f32,
   pub y: f32,
   pub size: f32,
   pub speed: f32,
   pub brightness: f32,
}

impl Star {
    pub fn new(x: f32, y: f32, size: f32, speed: f32, brightness: f32) -> Self {
        Star {
            x,
            y,
            size,
            speed,
            brightness,
        }
    }

    /// Default lower and upper bounds spanning the full screen, used when
    /// [`Starfield::new`](super::Starfield::new) is called without explicit limits.
    pub fn default_bounds() -> (Star, Star) {
        (
            Star::new(0.0, 0.0, 1.0, 10.0, 0.3),
            Star::new(screen_width(), screen_height(), 3.0, 40.0, 1.0),
        )
    }

    /// Advances the star downward by `speed * dt`. Once it drifts past the
    /// bottom edge it wraps back to the top at a fresh random `x`, so the
    /// field keeps scrolling indefinitely instead of running out of stars.
    pub fn update_star_position(&mut self){
        let dt = get_frame_time();
        self.y += self.speed * dt;
        if self.y > screen_height() {
            self.y = 0.0;
            self.x = rand::gen_range(0.0, screen_width());
        }
    }
}

impl Default for Star {
    fn default() -> Self {
        Star {
            x: rand::gen_range(0.0, screen_width()),
            y: rand::gen_range(0.0, screen_height()),
            size: rand::gen_range(1.0, 3.0),
            speed: rand::gen_range(10.0, 40.0),
            brightness: rand::gen_range(0.3, 1.0),
        }
    }
}

impl Drawable for Star {
    fn draw(&self) {
        let color = Color::new(1.0, 1.0, 1.0, self.brightness);
        let spike_length = self.size * 1.5;
        let spike_width = (self.size * 0.4).max(1.0);

        draw_line(self.x - spike_length, self.y, self.x + spike_length, self.y, spike_width, color);
        draw_line(self.x, self.y - spike_length, self.x, self.y + spike_length, spike_width, color);
        draw_circle(self.x, self.y, self.size , color);
    }
}

impl StateUpdatable<()> for Star {

    fn update_state(&mut self, _data: ()) {
        // update star position
        self.update_star_position();
    }
}