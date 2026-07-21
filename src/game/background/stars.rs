use macroquad::prelude::*;

use crate::game::rendering::Drawable;

pub struct Star {
    x: f32,
    y: f32,
    size: f32,
    speed: f32,
    brightness: f32,
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
    /// [`generate_starfield`] is called without explicit limits.
    fn default_bounds() -> (Star, Star) {
        (
            Star::new(0.0, 0.0, 1.0, 10.0, 0.3),
            Star::new(screen_width(), screen_height(), 3.0, 40.0, 1.0),
        )
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

pub struct Starfield {
    stars: Vec<Star>
}

impl Drawable for Starfield {
    fn draw(&self) {
        for star in &self.stars {
            draw_circle(
                star.x,
                star.y,
                star.size,
                Color::new(1.0, 1.0, 1.0, star.brightness),
            );
        }
    }
}

/// Returns `(a, b)` reordered so the smaller value comes first, regardless
/// of which one was actually passed as "lower" or "upper".
fn ordered(a: f32, b: f32) -> (f32, f32) {
    (a.min(b), a.max(b))
}

/// Generates a starfield of `star_count` stars, with each field of every
/// star drawn uniformly from the range between the matching field in
/// `limits.0` and `limits.1` (order doesn't matter — each field is
/// normalized independently). Defaults to [`Star::default_bounds`] spanning
/// the full screen when `limits` is `None`.
pub fn generate_starfield(star_count: usize, limits: Option<(Star, Star)>) -> Starfield {
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

    Starfield { stars }
}
