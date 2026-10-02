pub mod bullet_direction;
pub mod bullet_kind;
pub mod bullet_stats;
pub mod bullet_textures;
pub mod utils;

use macroquad::prelude::*;

use crate::game::{
    animation::Animation,
    entities::bullet::{
        bullet_direction::BulletDirection, bullet_kind::BulletKind, bullet_stats::BulletStats,
    },
    traits::{
        object::{HasBoundingBox, HasBoundingCircle},
        rendering::{Drawable, StateUpdatable},
    },
};

pub struct Bullet {
    bounds: Rect,
    kind: BulletKind,
    /// 12:00 clock => 0 or 360 degree, facing upwards, 6:00 clock => 180 degree, facing downwards
    direction: f32,
    stats: BulletStats,
    animation: Animation,
}

impl Bullet {
    /// Creates a bullet of `kind` at `pos`, travelling in `direction`
    /// (`BulletDirection::default()` fires straight up).
    pub fn new(pos: Vec2, direction: BulletDirection, kind: BulletKind) -> Self {
        let animation = kind.animation();
        let size = *animation.frame_scale();
        Self {
            bounds: Rect {
                x: pos.x,
                y: pos.y,
                w: size.x,
                h: size.y,
            },
            stats: BulletStats::stats_for(kind),
            animation,
            kind,
            direction: direction.degrees(),
        }
    }

    /// Damage this bullet deals to whatever it hits.
    pub fn damage(&self) -> u32 {
        self.stats.damage()
    }

    /// Moves the bullet by `stats.speed() * dt` along `direction`.
    fn apply_movement(&mut self, dt: f32) {
        let dir_angle_in_degrees = self.direction;
        let dir_in_radians = dir_angle_in_degrees.to_radians();
        // On a unit circle, θ gives x = sin(θ) and y = cos(θ).
        // Negate y to account for screen coordinates increasing downward.
        let dir = Vec2::new(dir_in_radians.sin(), -dir_in_radians.cos());
        let pos = self.bounds.point() + dir * self.stats.speed() * dt;
        self.bounds.x = pos.x;
        self.bounds.y = pos.y;
    }
}

impl HasBoundingBox for Bullet {
    fn bounding_box(&self) -> Rect {
        self.bounds
    }
}

impl HasBoundingCircle for Bullet {
    /// Collision circle: centered on the bullet's bounding box, with radius
    /// = half the box's smaller dimension (same convention as `Ship` and
    /// `Asteroid`).
    fn bounding_circle(&self) -> Circle {
        let center = self.bounds.center();
        let radius = self.bounds.w.min(self.bounds.h) / 2.0;
        Circle::new(center.x, center.y, radius)
    }
}

impl StateUpdatable<()> for Bullet {
    fn update_state(&mut self, _data: ()) {
        self.apply_movement(get_frame_time());
    }
}

impl Drawable for Bullet {
    fn draw(&self) {
        draw_texture_ex(
            self.animation.current_frame(),
            self.bounds.x,
            self.bounds.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(self.bounds.size()),
                ..Default::default()
            },
        );
    }
}
