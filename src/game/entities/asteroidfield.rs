use crate::game::{
    entities::asteroidfield::asteroid::*, rendering::{Drawable, StateUpdatable}, utils::MinMax,
};
use macroquad::{math::Vec2, window::{screen_width}};

pub mod asteroid;

#[derive(Debug, Clone)]
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
    pub fn new(asteroid_count: usize, asteroid_kind: AsteroidKind, scale_limits: MinMax<f32>) -> AsteroidField {

        let asteroids = (0..asteroid_count)
            .map(|_| {
                let random_x_coordinate = macroquad::rand::gen_range(0.0, screen_width());
                let random_y_cordinate = macroquad::rand::gen_range(0.0, 10.0);
                let random_scale= macroquad::rand::gen_range(scale_limits.min, scale_limits.max);

                // New asteroid at near screen top  with 0 initial rotation, random scale and random x,y coordinate
                Asteroid::new(Vec2::new(random_x_coordinate, random_y_cordinate), 0.0, asteroid_kind, random_scale)
            })
            .collect();

        AsteroidField { asteroids }
    }
}

impl Default for AsteroidField {
    fn default() -> Self {
        AsteroidField::new(20, AsteroidKind::MoltenDarkAsteroid, MinMax{min: 0.3, max: 2.0})
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
