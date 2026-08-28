use macroquad::math::Rect;

use crate::game::entities::{
    bullet::bullet_stats::BulletStats,
    ship::{ship_kind::ShipKind, guns::Guns},
};

#[derive(Debug, Clone)]
pub struct ShipStats {
    max_health: f32,
    cur_health: f32,
    speed: f32,
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

    /// Reduces `cur_health` by `amount`, clamped at 0.
    pub fn apply_damage(&mut self, amount: f32) {
        self.cur_health = (self.cur_health - amount).max(0.0);
    }

    pub fn stats_for(ship_kind: ShipKind) -> ShipStats {
        match ship_kind {
            ShipKind::Vanguard => ShipStats {
                max_health: 100.0,
                cur_health: 100.0,
                speed: 200.0,
            },
            ShipKind::Sentinel => ShipStats {
                max_health: 150.0,
                cur_health: 150.0,
                speed: 120.0,
            },
            ShipKind::Viper => ShipStats {
                max_health: 60.0,
                cur_health: 60.0,
                speed: 280.0,
            },
        }
    }

    /// Stats shown on the ship-select preview card, as label/value pairs.
    pub fn preview_stats_for(ship_kind: ShipKind) -> Vec<(String, String)> {
        let stats = ShipStats::stats_for(ship_kind);
        let bullet_damage = BulletStats::stats_for(ship_kind.bullet_kind()).damage();
        // Bounds don't affect gun count, so an empty `Rect` is sufficient here.
        let gun_count = Guns::guns_for_ship(ship_kind, Rect::new(0.0, 0.0, 0.0, 0.0)).gun_count();
        vec![
            ("Damage".to_string(), format!("{}", bullet_damage as i32)),
            ("Defense".to_string(), format!("{}", stats.max_health() as i32)),
            ("Speed".to_string(), format!("{}", stats.speed() as i32)),
            ("Guns".to_string(), format!("{}", gun_count)),
        ]
    }
}
