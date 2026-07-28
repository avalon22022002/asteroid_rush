use crate::game::{
    entities::asteroidfield::asteroid::*,
    rendering::{Drawable, StateUpdatable},
};

pub mod asteroid;

pub struct AsteroidField {
    asteroids: Vec<Asteroid>,
}

impl AsteroidField {
    /// Generates an asteroid field of `asteroid_count` asteroids, with each
    /// field of every asteroid drawn uniformly from the range between the
    /// matching field in `limits.0` and `limits.1` (order doesn't matter —
    /// each field is normalized independently). Defaults to
    /// [`Asteroid::default_bounds`] spanning the full screen width when
    /// `limits` is `None`.
    pub fn new(asteroid_count: usize, limits: Option<(Asteroid, Asteroid)>) -> AsteroidField {
        let (lower_limits, upper_limits) = limits.unwrap_or_else(Asteroid::default_bounds);

        let asteroids = (0..asteroid_count)
            .map(|_| Asteroid::random_between_range(&lower_limits, &upper_limits))
            .collect();

        AsteroidField { asteroids }
    }
}

impl Drawable for AsteroidField {
    fn draw(&self) {
        for asteroid in &self.asteroids {
            asteroid.draw();
        }
    }
}

impl StateUpdatable<()> for AsteroidField {
    fn update_state(&mut self, data: ()) {
        for asteroid in self.asteroids.iter_mut() {
            asteroid.update_state(data);
        }
    }
}
