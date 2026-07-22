use macroquad::prelude::*;

use crate::game::{rendering::Drawable, rendering::StateUpdatable, utils::ordered, background::starfield::star::*};

pub mod star;

pub struct Starfield {
    stars: Vec<Star>
}

impl Starfield{

    /// Generates a starfield of `star_count` stars, with each field of every
    /// star drawn uniformly from the range between the matching field in
    /// `limits.0` and `limits.1` (order doesn't matter — each field is
    /// normalized independently). Defaults to [`Star::default_bounds`] spanning
    /// the full screen when `limits` is `None`.
    pub fn new(star_count: usize, limits: Option<(Star, Star)>) -> Starfield {
        let (lower_limits, upper_limits) = limits.unwrap_or_else(Star::default_bounds);

        let (x_lo, x_hi) = ordered(lower_limits.x, upper_limits.x);
        let (y_lo, y_hi) = ordered(lower_limits.y, upper_limits.y);
        let (size_lo, size_hi) = ordered(lower_limits.size, upper_limits.size);
        let (speed_lo, speed_hi) = ordered(lower_limits.speed, upper_limits.speed);
        let (brightness_lo, brightness_hi) = ordered(lower_limits.brightness, upper_limits.brightness);

        let stars = (0..star_count)
            .map(|_| {
                Star::new(
                    rand::gen_range(x_lo, x_hi),
                    rand::gen_range(y_lo, y_hi),
                    rand::gen_range(size_lo, size_hi),
                    rand::gen_range(speed_lo, speed_hi),
                    rand::gen_range(brightness_lo, brightness_hi),
                )
            })
            .collect();

        return Starfield { stars }
    }

}

impl Drawable for Starfield {
    fn draw(&self) {
        for star in &self.stars {
            star.draw();
        }
    }
}

impl StateUpdatable<()> for Starfield {

    fn update_state(&mut self, _data: ()) {
        for star in self.stars.iter_mut() {
            // update star position
            star.update_star_position();
        }
    }
}