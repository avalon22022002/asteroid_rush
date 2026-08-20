use crate::game::{
    entities::{ship::{ShipKind}, asteroidfield::AsteroidField}
};
pub enum GameLevels {
    Level1,
    Level2,
    Level3
}

pub struct GameConfig {
    level: GameLevels,
    ship_kind: ShipKind,
    asteroid_field: AsteroidField
}

impl Default for GameConfig {
    fn default() -> Self {
        GameConfig { 
            level: GameLevels::Level1,
            ship_kind: ShipKind::Sentinel,
            asteroid_field: AsteroidField::default()
        }
    }
}