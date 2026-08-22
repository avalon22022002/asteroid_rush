use macroquad::prelude::*;

use crate::game::{
    game_config::GameLevel,
    asset_repository::{
        sprite_repository::{AsteroidV1Textures, SpriteRepository, traits::{SpriteBounds, SpriteTextures}}, traits::Singleton,
    }, entities::animation::Animation, rendering::{Drawable, StateUpdatable}, utils::{MinMax, biased_random_in_range}
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

pub struct Asteroid {
    pos: Vec2, // Position (x, y) of the asteroid
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
        Self {
            pos,
            current_rotation,
            kind,
            scale,
            animation: Animation::new(
                asteroid_sprites.get_textures_for(&kind.texture_kind()),
                kind.texture_kind().content_size_at_logical_unit_scale() * scale,
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
                    self.pos.x = rand::gen_range(0.0, screen_width());
                    self.status = AsteroidStatus::Active;
                }
                Some(new_remaining) => *remaining = new_remaining,
            }
            // Still Spawning this frame (or just became active) — skip the
            // falling/rotation logic below until next frame.
            return;
        }

        self.pos.y += self.stats.speed * dt;
        self.current_rotation += self.stats.rotation_speed * dt;
        if self.pos.y > screen_height() {
            self.pos.y = 0.0;
            self.pos.x = rand::gen_range(0.0, screen_width());
            self.status = AsteroidStatus::Spawning { remaining: self.stats.spawn_time };
        }
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

impl Drawable for Asteroid {
    fn draw(&self) {
        // Not on screen yet — nothing to draw until its delay elapses.
        if matches!(self.status, AsteroidStatus::Spawning { .. }) {
            return;
        }
        draw_texture_ex(
            self.animation.current_frame(),
            self.pos.x,
            self.pos.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(*self.animation.frame_scale()),
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
