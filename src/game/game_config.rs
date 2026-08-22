use crate::game::{
    entities::{ship::{ShipKind}, asteroidfield::AsteroidField}
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameLevel {
    Level1,
    Level2,
    Level3
}

pub struct GameConfig {
    level: GameLevel,
    ship_kind: ShipKind,
    asteroid_field: AsteroidField
}

impl Default for GameConfig {
    fn default() -> Self {
        GameConfig { 
            level: GameLevel::Level1,
            ship_kind: ShipKind::Sentinel,
            asteroid_field: AsteroidField::default()
        }
    }
}