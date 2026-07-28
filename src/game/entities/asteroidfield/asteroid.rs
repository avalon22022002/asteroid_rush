use std::sync::OnceLock;

use macroquad::prelude::*;

use crate::game::{
    rendering::{Drawable, StateUpdatable},
    utils::ordered,
};

const LOG_PREFIX: &str = "[asteroid]";

/// Identifies which asteroid texture to draw. Add a variant here (and cases
/// in `AsteroidTextureKind::texture`/`render_size`) to register a new
/// asteroid look.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AsteroidTextureKind {
    /// Dark rock veined with glowing molten cracks.
    Molten,
}

impl AsteroidTextureKind {
    /// Decodes this kind's texture. Loaded once per kind and reused after
    /// that — `include_bytes!` bakes the file in at compile time, so a
    /// missing/renamed asset fails the build instead of surfacing as a
    /// runtime error.
    fn texture(self) -> &'static Texture2D {
        match self {
            AsteroidTextureKind::Molten => {
                static TEXTURE: OnceLock<Texture2D> = OnceLock::new();
                TEXTURE.get_or_init(|| {
                    println!(
                        "{LOG_PREFIX} loading {self:?} (assets/animations/asteroid/asteroid1.png)..."
                    );
                    Texture2D::from_file_with_format(
                        include_bytes!("../../../../assets/animations/asteroid/asteroid_0.png"),
                        None,
                    )
                })
            }
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
        Self {
            x,
            y,
            kind,
            scale,
            speed,
            rotation,
            rotation_speed,
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
        Asteroid {
            x: rand::gen_range(0.0, screen_width()),
            y: rand::gen_range(0.0, screen_height()),
            kind: AsteroidTextureKind::Molten,
            scale: rand::gen_range(0.5, 1.5),
            speed: rand::gen_range(60.0, 220.0),
            rotation: rand::gen_range(0.0, std::f32::consts::TAU),
            rotation_speed: rand::gen_range(-std::f32::consts::PI, std::f32::consts::PI),
        }
    }
}

impl Drawable for Asteroid {
    fn draw(&self) {
        let texture = self.kind.texture();
        let size = Vec2::splat(self.kind.render_size() * self.scale);
        draw_texture_ex(
            texture,
            self.x,
            self.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(size),
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