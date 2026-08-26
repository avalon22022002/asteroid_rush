pub mod ship_kind;
pub mod ship_stats;
mod utils;

use macroquad::prelude::*;

use crate::game::{
    BASE_HEIGHT, BASE_WIDTH,
    animation::Animation,
    entities::ship::{ship_kind::ShipKind, ship_stats::ShipStats, utils::user_input::movement_input},
    traits::{
        object::{HasBoundingBox, HasBoundingCircle},
        rendering::{Drawable, StateUpdatable},
    },
};

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
    /// Creates a ship of `kind` that fits as closely as possible inside
    /// `approx_bounds` while preserving the sprite's aspect ratio (no cropping, stretching, or squashing).
    ///
    /// `approx_bounds` is the area where the caller wants the ship to appear.
    /// The ship's actual on-screen bounds may be smaller on one axis so that
    /// the sprite keeps its original shape without being stretched, squashed,
    /// or cropped.
    pub fn new(approx_bounds: Rect, kind: ShipKind) -> Self {
        let (alive_animation, dead_animation, bounds) =
            kind.get_alive_and_dead_animations_for(approx_bounds);

        Self {
            bounds,
            kind,
            ship_stats: ShipStats::stats_for(kind),
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
        self.ship_stats.apply_damage(amount);
        if self.ship_stats.cur_health() == 0.0 {
            self.is_alive = false;
        }
    }

    /// Moves the ship by this frame's arrow-key input at `ship_stats.speed`
    /// units/second, clamped so it can't drift outside the game's logical
    /// `BASE_WIDTH`x`BASE_HEIGHT` bounds. Only `bounds`' position moves —
    /// its size was fixed at construction.
    fn apply_movement(&mut self, dt: f32) {
        let dir = movement_input();
        let pos = (self.bounds.point() + dir * self.ship_stats.speed() * dt).clamp(
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
