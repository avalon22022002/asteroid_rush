use crate::game::{background::starfield::star::*, rendering::Drawable, rendering::StateUpdatable};

pub mod star;

pub struct Starfield {
    stars: Vec<Star>,
}

impl Starfield {
    /// Generates a starfield of `star_count` stars, with each field of every
    /// star drawn uniformly from the range between the matching field in
    /// `limits.0` and `limits.1` (order doesn't matter — each field is
    /// normalized independently). Defaults to [`Star::default_bounds`] spanning
    /// the full screen when `limits` is `None`.
    pub fn new(star_count: usize, limits: Option<(Star, Star)>) -> Starfield {
        let (lower_limits, upper_limits) = limits.unwrap_or_else(Star::default_bounds);

        let stars = (0..star_count)
            .map(|_| Star::random_between_range(&lower_limits, &upper_limits))
            .collect();

        Starfield { stars }
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
