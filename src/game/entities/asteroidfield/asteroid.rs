pub mod asteroid_kind;
pub mod asteroid_stats;

use macroquad::prelude::*;

use crate::game::{
    animation::Animation,
    asset_repository::{
        sprite_repository::{SpriteRepository, traits::{SpriteBounds, SpriteTextures}}, traits::Singleton,
    },
    blink::Blink,
    entities::asteroidfield::asteroid::{asteroid_kind::AsteroidKind, asteroid_stats::AsteroidStats},
    traits::{
        damage::{Damageable, DamageResult},
        object::{HasBoundingBox, HasBoundingCircle},
        rendering::{Drawable, StateUpdatable},
    }
};

/// How long an asteroid flashes for after taking damage, in seconds.
const HIT_BLINK_DURATION: f32 = 0.6;
/// How long each visible/invisible phase lasts while flashing, in seconds.
const HIT_BLINK_INTERVAL: f32 = 0.08;

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
    /// Flashes briefly after taking damage; see `HIT_BLINK_DURATION`.
    blink: Blink,
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
        let stats = AsteroidStats::random_biased_by_scale_for(kind, scale);
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
            status: AsteroidStatus::Spawning { remaining: stats.spawn_time() },
            stats,
            blink: Blink::new(HIT_BLINK_INTERVAL),
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

        self.bounds.y += self.stats.speed() * dt;
        self.current_rotation += self.stats.rotation_speed() * dt;
        if self.bounds.y > screen_height() {
            self.respawn();
        }
    }

    /// Resets this asteroid back to the top of the screen at a fresh random
    /// `x` with health restored to full, re-entering `Spawning` for another
    /// `spawn_time`-frame delay. Used both when an asteroid drifts off the
    /// bottom of the screen and when one is destroyed (e.g. by colliding
    /// with the ship, or having its health depleted by bullets).
    pub fn respawn(&mut self) {
        self.bounds.y = 0.0;
        self.bounds.x = rand::gen_range(0.0, screen_width());
        self.status = AsteroidStatus::Spawning { remaining: self.stats.spawn_time() };
        self.stats.reset_health();
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

impl Damageable for Asteroid {
    /// Reduces this asteroid's health by `amount`, respawning (destroying)
    /// it once health reaches 0. Otherwise flashes briefly to signal the hit.
    fn take_damage(&mut self, amount: u32) -> DamageResult {
        self.stats.apply_damage(amount);
        if self.stats.cur_health() == 0 {
            self.respawn();
            DamageResult::Destroyed
        } else {
            self.blink.trigger(HIT_BLINK_DURATION);
            DamageResult::Alive
        }
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
        if !self.is_active() || !self.blink.is_visible() {
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
        let dt = get_frame_time();
        self.update_position(dt);
        self.blink.advance(dt);
    }
}
