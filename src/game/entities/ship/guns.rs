use macroquad::prelude::*;

use crate::game::{
    BASE_HEIGHT, BASE_WIDTH, entities::{
        asteroidfield::AsteroidField, bullet::{Bullet, bullet_direction::BulletDirection}, ship::ship_kind::ShipKind,
    }, traits::{object::HasBoundingBox, rendering::{Drawable, StateUpdatable}},
};

/// Manages a ship's guns, firing cooldown, and active bullets.
///
/// Gun count and positions are defined per ship kind. Bullet damage comes
/// from each bullet's BulletStats, avoiding duplicated weapon data.
pub struct Guns {
    /// Gun Muzzle positions , updated with ship's movement
    gun_positions: Vec<Vec2>,
    /// Seconds between shots — how long `cur_cooldown` resets to after firing.
    max_cooldown: f32,
    /// Counts down to 0 between shots. Starts at 0 so a freshly-created ship
    /// can fire immediately.
    cur_cooldown: f32,
    bullets: Vec<Bullet>,
}

impl Guns {
    /// Builds the gun loadout for `ship_kind`: how many guns and how fast
    /// they fire.
    pub fn guns_for_ship(ship_kind: ShipKind, ship_bounds: Rect) -> Guns {
        let max_cooldown= match ship_kind {
            ShipKind::Vanguard => 0.3,
            ShipKind::Sentinel =>  0.45,
            ShipKind::Viper => 0.2,
        };

        Guns {
            gun_positions: Guns::gun_muzzle_positions(ship_kind, ship_bounds),
            max_cooldown,
            cur_cooldown: 0.0,
            bullets: Vec::new(),
        }
    }

    /// Returns the world-space muzzle position for each gun on the ship.
    fn gun_muzzle_positions(ship_kind: ShipKind, ship_bounds: Rect) -> Vec<Vec2> {
        match ship_kind {
            // Sentinel has 4 barrels: the 2 outer ones flush with the top of
            // the sprite, the 2 middle ones recessed behind those, so they
            // fire from lower down — 1.2x the outer barrels' y.
            ShipKind::Sentinel => vec![
                Vec2::new(ship_bounds.x + ship_bounds.w * 0.3, ship_bounds.y),
                Vec2::new(ship_bounds.x + ship_bounds.w * 0.45, ship_bounds.y * 1.2),
                Vec2::new(ship_bounds.x + ship_bounds.w * 0.55, ship_bounds.y * 1.2),
                Vec2::new(ship_bounds.x + ship_bounds.w * 0.65, ship_bounds.y),
            ],
            // Single nose-mounted gun, centered — placeholder until each
            // kind's real sprite and gun layout are added.
            ShipKind::Vanguard => vec![Vec2::new(ship_bounds.x + ship_bounds.w * 0.5, ship_bounds.y)],
            ShipKind::Viper => vec![Vec2::new(ship_bounds.x + ship_bounds.w * 0.5, ship_bounds.y)],
        }
    }

    /// Number of muzzles this loadout fires from.
    pub fn gun_count(&self) -> usize {
        self.gun_positions.len()
    }

    /// Counts the cooldown down by `dt`, clamped at 0.
    pub fn tick_cooldown(&mut self, dt: f32) {
        self.cur_cooldown = (self.cur_cooldown - dt).max(0.0);
    }

    /// Whether enough time has passed since the last shot to fire again.
    pub fn ready_to_fire(&self) -> bool {
        self.cur_cooldown == 0.0
    }

    /// Fires one bullet of `ship_kind`'s bullet kind from each muzzle
    /// position within `ship_bounds`, and restarts the cooldown. Positions
    /// are re-resolved from `ship_bounds` here (rather than reusing
    /// `gun_positions`, which is fixed at construction time) since the ship
    /// has likely moved since then.
    pub fn fire(&mut self, ship_kind: ShipKind, ship_bounds: Rect) {
        let bullet_kind = ship_kind.bullet_kind();
        for pos in Guns::gun_muzzle_positions(ship_kind, ship_bounds) {
            self.bullets.push(Bullet::new(pos, BulletDirection::Up, bullet_kind));
        }
        self.cur_cooldown = self.max_cooldown;
    }

    /// Advances every in-flight bullet and drops the ones that have left
    /// the screen, so the list doesn't grow unbounded.
    pub fn update(&mut self) {
        for bullet in self.bullets.iter_mut() {
            bullet.update_state(());
        }
        self.bullets.retain(|bullet| {
            let bounds = bullet.bounding_box();
            bounds.y + bounds.h > 0.0
                && bounds.y < BASE_HEIGHT
                && bounds.x + bounds.w > 0.0
                && bounds.x < BASE_WIDTH
        });
    }

    /// Checks every in-flight bullet against `asteroid_field`, damaging
    /// whichever asteroid each hits, and drops any bullet that hit
    /// something — a bullet is consumed on impact.
    pub fn resolve_collisions(&mut self, asteroid_field: &mut AsteroidField) {
        self.bullets.retain(|bullet| {
            !asteroid_field.resolve_bullet_collision(bullet, bullet.damage())
        });
    }
}

impl Drawable for Guns {
    fn draw(&self) {
        for bullet in &self.bullets {
            bullet.draw();
        }
    }
}
