use macroquad::prelude::*;

use crate::game::{
    BASE_HEIGHT, BASE_WIDTH,
    asset_repository::{
        sprite_repository::{traits::{SpriteTextures, SpriteBounds}, SpriteRepository, ShipV1Textures},
        traits::Singleton
    },
    animation::Animation,
    object::{HasBoundingBox, HasBoundingCircle},
    rendering::{Drawable, StateUpdatable},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShipKind {
    // Allrounder: 1 Gun, Medium Health, Medium Speed
    Vanguard,
    // Defender: 3 Guns, High Health, Slow
    Sentinel,
    // Attacker: 3 Guns, Low Health, Fast
    Viper,
}

impl ShipKind {
    /// Display name shown in the ship-select UI.
    pub fn display_name(&self) -> &'static str {
        match self {
            ShipKind::Vanguard => "Vanguard",
            ShipKind::Sentinel => "Sentinel",
            ShipKind::Viper => "Viper",
        }
    }

    /// One-line role blurb shown under `display_name` in the ship-select UI.
    pub fn role(&self) -> &'static str {
        match self {
            ShipKind::Vanguard => "Allrounder",
            ShipKind::Sentinel => "Defender",
            ShipKind::Viper => "Attacker",
        }
    }

    /// Base `ShipStats` for a freshly spawned ship of this kind, matching
    /// the class blurbs above (gun count, relative health, relative speed).
    /// Also drives the ship-select preview, so it's public.
    pub fn stats(&self) -> ShipStats {
        match self {
            ShipKind::Vanguard => ShipStats {
                max_health: 100.0,
                cur_health: 100.0,
                speed: 200.0,
                gun_count: 1,
                fire_damage: 10.0,
            },
            ShipKind::Sentinel => ShipStats {
                max_health: 150.0,
                cur_health: 150.0,
                speed: 120.0,
                gun_count: 2,
                fire_damage: 8.0,
            },
            ShipKind::Viper => ShipStats {
                max_health: 60.0,
                cur_health: 60.0,
                speed: 280.0,
                gun_count: 3,
                fire_damage: 14.0,
            },
        }
    }

    /// Stats shown on the ship-select preview card, as label/value pairs.
    pub fn preview_stats(&self) -> Vec<(String, String)> {
        let stats = self.stats();
        vec![
            ("Damage".to_string(), format!("{}", stats.fire_damage() as i32)),
            ("Defense".to_string(), format!("{}", stats.max_health() as i32)),
            ("Speed".to_string(), format!("{}", stats.speed() as i32)),
            ("Guns".to_string(), format!("{}", stats.gun_count())),
        ]
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ShipStats {
    max_health: f32,
    cur_health: f32,
    speed: f32,
    gun_count: u8,
    fire_damage: f32,
}

impl ShipStats {
    pub fn max_health(&self) -> f32 {
        self.max_health
    }
    pub fn speed(&self) -> f32 {
        self.speed
    }
    pub fn cur_health(&self) -> f32 {
        self.cur_health
    }
    pub fn gun_count(&self) -> u8 {
        self.gun_count
    }
    pub fn fire_damage(&self) -> f32 {
        self.fire_damage
    }
}
pub struct Ship {
    /// The ship's on-screen box — position and size together. Only `x`/`y`
    /// change after construction (see `apply_movement`); `w`/`h` are fixed
    /// once `new` fits the sprite into the bounds it was given.
    bounds: Rect,
    kind: ShipKind,
    ship_stats: ShipStats,
    alive_animation: Animation,
    dead_animation: Animation,
    is_alive: bool,
    /// When `true`, the ship ignores player input entirely — no movement,
    /// and (once added) no shooting either. For ships that are only ever
    /// drawn for show, e.g. the one on the home page.
    locked: bool,
}

impl Ship {
    /// Creates the alive and dead animations for `kind`, sized to fit as closely
    /// as possible inside `approx_bounds` without stretching, squashing, or
    /// cropping the sprite. Also returns that fitted box, which becomes the
    /// ship's starting `bounds`.
    pub fn get_alive_and_dead_animations_for(
        kind: ShipKind,
        approx_bounds: Rect,
    ) -> (Animation, Animation, Rect) {
        let ship_sprites = &SpriteRepository::get_instance().ship_v1_sprite;

        match kind {
            ShipKind::Sentinel => {
                let sprite = ShipV1Textures::SentinelAlive;
                let fitted_bounds = sprite.fit_centered_in(approx_bounds);
                let crop = Some(sprite.content_bounds());

                let alive = Animation::new(
                    ship_sprites.get_textures_for(&sprite),
                    fitted_bounds.size(),
                    12.0,
                    crop,
                );

                // No dedicated death sprite set yet — freeze on the last
                // alive frame as a placeholder until one is added.
                let dead = Animation::new(
                    ship_sprites.get_textures_for(&sprite),
                    fitted_bounds.size(),
                    1.0,
                    crop,
                );

                (alive, dead, fitted_bounds)
            }

            ShipKind::Vanguard => todo!("vanguard animation frames not added yet"),
            ShipKind::Viper => todo!("viper animation frames not added yet"),
        }
    }

    /// Creates a ship of `kind` that fits as closely as possible inside
    /// `approx_bounds` while preserving the sprite's aspect ratio (no cropping, stretching, or squashing).
    ///
    /// `approx_bounds` is the area where the caller wants the ship to appear.
    /// The ship's actual on-screen bounds may be smaller on one axis so that
    /// the sprite keeps its original shape without being stretched, squashed,
    /// or cropped.
    pub fn new(approx_bounds: Rect, kind: ShipKind) -> Self {
        let (alive_animation, dead_animation, bounds) =
            Self::get_alive_and_dead_animations_for(kind, approx_bounds);

        Self {
            bounds,
            kind,
            ship_stats: kind.stats(),
            alive_animation,
            dead_animation,
            is_alive: true,
            locked: false,
        }
    }

    /// Builder-style: sets whether this ship ignores player input (see
    /// `locked`). Chain onto `new`, e.g. `Ship::new(..).locked(true)`.
    pub fn locked(mut self, locked: bool) -> Self {
        self.locked = locked;
        self
    }

    /// Whether the ship is currently alive (i.e. hasn't been destroyed).
    pub fn is_alive(&self) -> bool {
        self.is_alive
    }

    /// Reduces `cur_health` by `amount`, clamped at 0, and marks the ship
    /// dead once health reaches 0. A no-op if the ship is already dead.
    pub fn take_damage(&mut self, amount: f32) {
        if !self.is_alive {
            return;
        }
        self.ship_stats.cur_health = (self.ship_stats.cur_health - amount).max(0.0);
        if self.ship_stats.cur_health == 0.0 {
            self.is_alive = false;
        }
    }

    /// Moves the ship by this frame's arrow-key input at `ship_stats.speed`
    /// units/second, clamped so it can't drift outside the game's logical
    /// `BASE_WIDTH`x`BASE_HEIGHT` bounds. Only `bounds`' position moves —
    /// its size was fixed at construction.
    fn apply_movement(&mut self, dt: f32) {
        let dir = movement_input();
        let pos = (self.bounds.point() + dir * self.ship_stats.speed * dt).clamp(
            Vec2::ZERO,
            Vec2::new(BASE_WIDTH - self.bounds.w, BASE_HEIGHT - self.bounds.h),
        );
        self.bounds.x = pos.x;
        self.bounds.y = pos.y;
    }

    /// The animation that reflects the ship's current `is_alive` state.
    fn current_animation(&self) -> &Animation {
        if self.is_alive {
            &self.alive_animation
        } else {
            &self.dead_animation
        }
    }

    fn current_animation_mut(&mut self) -> &mut Animation {
        if self.is_alive {
            &mut self.alive_animation
        } else {
            &mut self.dead_animation
        }
    }
}

/// Reads the arrow keys and returns a direction vector for this frame's
/// movement input, normalized so diagonal movement isn't faster than
/// moving along a single axis. Zero if no arrow key is held.
fn movement_input() -> Vec2 {
    let mut dir = Vec2::ZERO;
    if is_key_down(KeyCode::Left) {
        dir.x -= 1.0;
    }
    if is_key_down(KeyCode::Right) {
        dir.x += 1.0;
    }
    if is_key_down(KeyCode::Up) {
        dir.y -= 1.0;
    }
    if is_key_down(KeyCode::Down) {
        dir.y += 1.0;
    }
    if dir != Vec2::ZERO {
        dir = dir.normalize();
    }
    dir
}

impl HasBoundingBox for Ship {
    fn bounding_box(&self) -> Rect {
        self.bounds
    }
}

impl HasBoundingCircle for Ship {
    /// Collision circle: centered on bounding box,
    /// with radius = half the box's smaller dimension.
    /// - Circle always stays inside the box, so it never registers a hit
    ///   the sprite's box wouldn't.
    /// - On the longer axis, the circle doesn't reach the bounding box's edges,
    ///   so some of that space isn't covered by the circle.
    fn bounding_circle(&self) -> Circle {
        let center = self.bounds.center();
        let radius = self.bounds.w.min(self.bounds.h) / 2.0;
        Circle::new(center.x, center.y, radius)
    }
}

impl Drawable for Ship {
    fn draw(&self) {
        draw_texture_ex(
            self.current_animation().current_frame(),
            self.bounds.x,
            self.bounds.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(self.bounds.size()),
                source: self.current_animation().frame_crop(),
                ..Default::default()
            },
        );
    }
}

impl StateUpdatable<()> for Ship {
    /// Only responds to movement input while alive and unlocked — a dead
    /// ship shouldn't steer (just play out its death animation in place),
    /// and a `locked` ship ignores player input altogether.
    fn update_state(&mut self, _data: ()) {
        let dt = get_frame_time();
        if self.is_alive && !self.locked {
            self.apply_movement(dt);
        }
        self.current_animation_mut().advance(dt);
    }
}
