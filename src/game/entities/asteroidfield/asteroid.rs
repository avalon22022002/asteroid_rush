use macroquad::prelude::*;

use crate::game::{
    game_config::GameLevel,
    asset_repository::{
        sprite_repository::{AsteroidV1Textures, SpriteRepository, traits::{SpriteBounds, SpriteTextures}}, traits::Singleton,
    }, animation::Animation, traits::object::{HasBoundingBox, HasBoundingCircle}, traits::rendering::{Drawable, StateUpdatable}, utils::{MinMax, biased_random_in_range}
};

/// Identifies which asteroid texture to draw. Add a variant here (and a
/// case in `AsteroidKind::texture_kind`) to register a new asteroid look.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AsteroidKind {
    /// Dark rock veined with glowing molten cracks.
    MoltenDarkAsteroid,
}

impl AsteroidKind {
    pub fn asteroid_kind_from_level(level: &GameLevel) -> AsteroidKind {
        match level {
            GameLevel::Level1 => AsteroidKind::MoltenDarkAsteroid,
            GameLevel::Level2 => AsteroidKind::MoltenDarkAsteroid,
            GameLevel::Level3 => AsteroidKind::MoltenDarkAsteroid,
        }
    }

    /// This kind's texture group in `SpriteRepository`.
    pub fn texture_kind(&self) -> AsteroidV1Textures {
        match self {
            AsteroidKind::MoltenDarkAsteroid => AsteroidV1Textures::MoltenDark,
        }
    }

    pub fn stat_range(&self) -> MinMax<AsteroidStats>{
        match self {
            AsteroidKind::MoltenDarkAsteroid =>  MinMax {
                min: AsteroidStats { speed: 50.0, rotation_speed: 1.6, health: 20, damage_on_collision: 10, spawn_time: 10 },
                max: AsteroidStats { speed: 120.0, rotation_speed: 4.2, health: 40, damage_on_collision: 25, spawn_time: 500},
            },
        }
    }
    fn random_stats_biased_by_scale(&self, scale: f32)-> AsteroidStats{
        let stat_range= self.stat_range();
        AsteroidStats {
            // Bigger asteroids are slower: flip the sign so growing size pulls toward min.
            speed: biased_random_in_range(MinMax { min:stat_range.min.speed, max: stat_range.max.speed }, -scale),
            // Bigger asteroids rotate slower: flip the sign so growing size pulls toward min.
            rotation_speed: biased_random_in_range(MinMax { min: stat_range.min.rotation_speed, max: stat_range.max.rotation_speed }, -scale),
            // Bigger asteroids take more hits to destroy: bias grows with size.
            health: biased_random_in_range(MinMax { min: stat_range.min.health as f32, max: stat_range.max.health as f32 }, scale) as u32,
            // Bigger asteroids deal more collision damage: bias grows with size.
            damage_on_collision: biased_random_in_range(MinMax { min: stat_range.min.damage_on_collision as f32, max: stat_range.max.damage_on_collision as f32 }, scale) as u32,
            // Bigger asteroids take longer to spawn: bias grows with scale.
            spawn_time: biased_random_in_range(MinMax { min: stat_range.min.spawn_time as f32, max: stat_range.max.spawn_time as f32 }, scale) as u32

        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Self::MoltenDarkAsteroid => "Molten Dark Asteroid"
        }
    }

    pub fn difficulty_label(&self) -> &'static str {
        match self {
            Self::MoltenDarkAsteroid => "Beginner Level Asteroid"
        }
    }

    /// Stats shown on the level-select preview card, as label/value pairs.
    pub fn preview_stats(&self) -> Vec<(String, String)> {
        let max_stats = self.stat_range().max;
        vec![
            ("Max speed".to_string(), format!("{}", max_stats.speed())),
            ("Max rotation speed".to_string(), format!("{}", max_stats.rotation_speed())),
            ("Max health".to_string(), format!("{}", max_stats.health())),
            ("Max collision damage".to_string(), format!("{}", max_stats.damage_on_collision())),
        ]
    }
}

#[derive(Debug, Clone)]
pub struct AsteroidStats {
    speed: f32,
    rotation_speed: f32,
    health: u32,
    damage_on_collision: u32,
    spawn_time: u32,
}

impl AsteroidStats {
    pub fn speed(&self) -> f32 {
        self.speed
    }
    pub fn rotation_speed(&self) -> f32 {
        self.rotation_speed
    }
    pub fn health(&self) -> u32 {
        self.health
    }
    pub fn damage_on_collision(&self) -> u32 {
        self.damage_on_collision
    }
}

/// Where an asteroid is in its spawn lifecycle.
#[derive(Debug, Clone, Copy)]
enum AsteroidStatus {
    /// Not yet on screen — counts `remaining` frames down to 0, then spawns
    /// (picks a fresh `x` and switches to `Active`). Used to stagger a
    /// freshly-created batch of asteroids so they don't all pop in at once.
    Spawning { remaining: u32 },
    Active,
}

#[derive(Debug, Clone)]
pub struct Asteroid {
    /// The asteroid's on-screen box (`x`, `y`, `w`, `h` — position and size together)
    bounds: Rect,
    current_rotation: f32, // Current rotation in radians (matches `DrawTextureParams::rotation`)
    kind: AsteroidKind, // The asteroid's kind
    scale: f32, // Scale factor for asteroid size variation, without needing separate art per size
    stats: AsteroidStats,
    animation: Animation,
    status: AsteroidStatus,
}

impl Asteroid {
    /// `spawn_delay_frames` postpones this asteroid's first appearance by
    /// that many frames instead of spawning it immediately — 0 (or less)
    /// spawns right away. Used to stagger a freshly-created batch of
    /// asteroids so they don't all pop in at once.
    pub fn new(
        pos: Vec2,
        current_rotation: f32,
        kind: AsteroidKind,
        scale: f32,
    ) -> Self {
        let asteroid_sprites = &SpriteRepository::get_instance().asteroid_v1_sprite;
        let stats = kind.random_stats_biased_by_scale(scale);
        let size = kind.texture_kind().content_size_at_logical_unit_scale() * scale;
        Self {
            bounds: Rect::new(pos.x, pos.y, size.x, size.y),
            current_rotation,
            kind,
            scale,
            animation: Animation::new(
                asteroid_sprites.get_textures_for(&kind.texture_kind()),
                size,
                1.0,
                None,
            ),
            status: AsteroidStatus::Spawning { remaining: stats.spawn_time },
            stats,
        }
    }

    /// Advances the asteroid downward by `speed * dt` and spins it by
    /// `rotation_speed * dt`. Once it drifts past the bottom edge it wraps
    /// back to the top at a fresh random `x` and re-enters `Spawning` for
    /// another `spawn_time`-frame delay, so the field keeps producing
    /// asteroids indefinitely instead of running out, staggered the same
    /// way a freshly-created batch is.
    ///
    /// While `Spawning`, this instead just counts the remaining frames down;
    /// once it hits 0 the asteroid picks a fresh `x` and starts falling from
    /// the next frame on.
    pub fn update_position(&mut self, dt: f32) {
        if let AsteroidStatus::Spawning { remaining } = &mut self.status {
            match remaining.checked_sub(1) {
                Some(0) | None => {
                    // Countdown finished — spawn now at a fresh x.
                    self.bounds.x = rand::gen_range(0.0, screen_width());
                    self.status = AsteroidStatus::Active;
                }
                Some(new_remaining) => *remaining = new_remaining,
            }
            // Still Spawning this frame (or just became active) — skip the
            // falling/rotation logic below until next frame.
            return;
        }

        self.bounds.y += self.stats.speed * dt;
        self.current_rotation += self.stats.rotation_speed * dt;
        if self.bounds.y > screen_height() {
            self.respawn();
        }
    }

    /// Resets this asteroid back to the top of the screen at a fresh random
    /// `x`, re-entering `Spawning` for another `spawn_time`-frame delay.
    /// Used both when an asteroid drifts off the bottom of the screen and
    /// when one is destroyed (e.g. by colliding with the ship).
    pub fn respawn(&mut self) {
        self.bounds.y = 0.0;
        self.bounds.x = rand::gen_range(0.0, screen_width());
        self.status = AsteroidStatus::Spawning { remaining: self.stats.spawn_time };
    }

    /// Whether this asteroid is currently on screen and collidable (i.e. not
    /// still counting down in `Spawning`).
    pub fn is_active(&self) -> bool {
        matches!(self.status, AsteroidStatus::Active)
    }

    /// Damage this asteroid deals to anything it collides with.
    pub fn damage_on_collision(&self) -> u32 {
        self.stats.damage_on_collision()
    }
}

impl HasBoundingBox for Asteroid {
    fn bounding_box(&self) -> Rect {
        self.bounds
    }
}

impl Default for Asteroid {
    fn default() -> Self {
        Asteroid::new(
            Vec2::new(rand::gen_range(0.0, screen_width()), rand::gen_range(0.0, screen_height())),
            rand::gen_range(60.0, 220.0),
            AsteroidKind::MoltenDarkAsteroid,
            rand::gen_range(0.5, 1.5)
        )
    }
}

impl HasBoundingCircle for Asteroid {
    /// Collision circle: centered on the asteroid's bounding box
    /// with radius = half the box's smaller dimension.
    /// - Circle always stays inside the box, so it never registers a hit
    ///   the sprite's box wouldn't.
    /// - On the longer axis, the circle doesn't reach the box's edges, so
    ///   some of that space isn't covered by the circle.
    fn bounding_circle(&self) -> Circle {
        let center = self.bounds.center();
        let radius = self.bounds.w.min(self.bounds.h) / 2.0;
        Circle::new(center.x, center.y, radius)
    }
}

impl Drawable for Asteroid {
    fn draw(&self) {
        // Not on screen yet — nothing to draw until its delay elapses.
        if !self.is_active() {
            return;
        }
        draw_texture_ex(
            self.animation.current_frame(),
            self.bounds.x,
            self.bounds.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(self.bounds.size()),
                rotation: self.current_rotation,
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
