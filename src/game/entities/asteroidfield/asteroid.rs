use macroquad::prelude::*;

use crate::game::{
    asset_repository::{
        sprite_repository::{traits::SpriteTextures, SpriteRepository, AsteroidV1Textures},
        traits::Singleton,
    },
    entities::animation::Animation,
    rendering::{Drawable, StateUpdatable},
    utils::ordered,
};

/// Identifies which asteroid texture to draw. Add a variant here (and cases
/// in `AsteroidTextureKind::sprite_kind`/`render_size`) to register a new
/// asteroid look.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AsteroidTextureKind {
    /// Dark rock veined with glowing molten cracks.
    Molten,
}

impl AsteroidTextureKind {
    /// This kind's texture group in `SpriteRepository`.
    fn sprite_kind(self) -> AsteroidV1Textures {
        match self {
            AsteroidTextureKind::Molten => AsteroidV1Textures::Molten,
        }
    }

    /// Draw size in pixels at `scale` 1.0 — independent of this kind's
    /// source png's actual (much larger) resolution.
    fn render_size(self) -> f32 {
        match self {
            AsteroidTextureKind::Molten => 60.0,
        }
    }
}

pub struct Asteroid {
    x: f32,
    y: f32,
    kind: AsteroidTextureKind,
    /// Multiplier applied to the source texture's size, so asteroids vary in
    /// size without needing separate art per size.
    scale: f32,
    /// Fall speed in units/second.
    speed: f32,
    /// Current rotation in radians (matches `DrawTextureParams::rotation`).
    rotation: f32,
    /// Spin rate in radians/second. Can be negative to spin counterclockwise.
    rotation_speed: f32,
    animation: Animation,
}

impl Asteroid {
    pub fn new(
        x: f32,
        y: f32,
        kind: AsteroidTextureKind,
        scale: f32,
        speed: f32,
        rotation: f32,
        rotation_speed: f32,
    ) -> Self {
        let asteroid_sprites = &SpriteRepository::get_instance().asteroid_v1_sprite;
        let animation = Animation::new(
            asteroid_sprites.get_textures_for(&kind.sprite_kind()),
            Vec2::splat(kind.render_size() * scale),
            1.0,
        );
        Self {
            x,
            y,
            kind,
            scale,
            speed,
            rotation,
            rotation_speed,
            animation,
        }
    }

    /// Default lower and upper bounds spanning the full screen width, used
    /// when [`AsteroidField::new`](super::AsteroidField::new) generates a
    /// field without explicit limits.
    pub fn default_bounds() -> (Asteroid, Asteroid) {
        (
            Asteroid::new(
                0.0,
                0.0,
                AsteroidTextureKind::Molten,
                0.5,
                60.0,
                0.0,
                -std::f32::consts::PI,
            ),
            Asteroid::new(
                screen_width(),
                0.0,
                AsteroidTextureKind::Molten,
                1.5,
                220.0,
                std::f32::consts::TAU,
                std::f32::consts::PI,
            ),
        )
    }

    /// Generates a random asteroid with each numeric field drawn uniformly
    /// from the range between the matching field in `lower` and `upper`
    /// (order doesn't matter — each field is normalized independently).
    /// `kind` isn't a range, so the new asteroid just takes `lower`'s.
    pub fn random_between_range(lower: &Asteroid, upper: &Asteroid) -> Asteroid {
        let (x_lo, x_hi) = ordered(lower.x, upper.x);
        let (scale_lo, scale_hi) = ordered(lower.scale, upper.scale);
        let (speed_lo, speed_hi) = ordered(lower.speed, upper.speed);
        let (rotation_lo, rotation_hi) = ordered(lower.rotation, upper.rotation);
        let (rotation_speed_lo, rotation_speed_hi) =
            ordered(lower.rotation_speed, upper.rotation_speed);

        Asteroid::new(
            rand::gen_range(x_lo, x_hi),
            0.0,
            lower.kind,
            rand::gen_range(scale_lo, scale_hi),
            rand::gen_range(speed_lo, speed_hi),
            rand::gen_range(rotation_lo, rotation_hi),
            rand::gen_range(rotation_speed_lo, rotation_speed_hi),
        )
    }

    /// Advances the asteroid downward by `speed * dt` and spins it by
    /// `rotation_speed * dt`. Once it drifts past the bottom edge it wraps
    /// back to the top at a fresh random `x`, so the field keeps falling
    /// indefinitely instead of running out of asteroids.
    pub fn update_position(&mut self, dt: f32) {
        self.y += self.speed * dt;
        self.rotation += self.rotation_speed * dt;
        if self.y > screen_height() {
            self.y = 0.0;
            self.x = rand::gen_range(0.0, screen_width());
        }
    }
}

impl Default for Asteroid {
    fn default() -> Self {
        Asteroid::new(
            rand::gen_range(0.0, screen_width()),
            rand::gen_range(0.0, screen_height()),
            AsteroidTextureKind::Molten,
            rand::gen_range(0.5, 1.5),
            rand::gen_range(60.0, 220.0),
            rand::gen_range(0.0, std::f32::consts::TAU),
            rand::gen_range(-std::f32::consts::PI, std::f32::consts::PI),
        )
    }
}

impl Drawable for Asteroid {
    fn draw(&self) {
        draw_texture_ex(
            self.animation.current_frame(),
            self.x,
            self.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(*self.animation.frame_scale()),
                rotation: self.rotation,
                ..Default::default()
            },
        );
    }
}

impl StateUpdatable<()> for Asteroid {
    fn update_state(&mut self, _data: ()) {
        self.update_position(get_frame_time());
    }
}
