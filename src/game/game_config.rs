use crate::game::entities::{asteroidfield::asteroid::asteroid_kind::AsteroidKind, ship::ship_kind::ShipKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameLevel {
    Level1,
    Level2,
    Level3
}

impl GameLevel {
    /// Time allotted to complete this level, in seconds, counted down by the HUD's timer.
    pub fn duration_secs(&self) -> f32 {
        match self {
            GameLevel::Level1 => 60.0,
            GameLevel::Level2 => 120.0,
            GameLevel::Level3 => 210.0,
        }
    }

    /// `duration_secs` formatted as `M:SS`, for display in level-select UI.
    pub fn duration_label(&self) -> String {
        let total = self.duration_secs() as u32;
        format!("{}:{:02}", total / 60, total % 60)
    }
}

#[derive(Debug, Clone)]
pub struct GameConfig {
    level: GameLevel,
    ship_kind: ShipKind,
    asteroid_kind: AsteroidKind
}

impl Default for GameConfig {
    fn default() -> Self {
        GameConfig { 
            level: GameLevel::Level1,
            ship_kind: ShipKind::Sentinel,
            asteroid_kind: AsteroidKind::MoltenDarkAsteroid
        }
    }
}

impl GameConfig {
    pub fn level(&self) -> GameLevel {
        self.level
    }
    pub fn set_level(&mut self, level: GameLevel) {
        self.level = level;
    }

    pub fn ship_kind(&self) -> ShipKind {
        self.ship_kind
    }

    pub fn set_ship_kind(&mut self, kind: ShipKind) {
        self.ship_kind = kind;
    }

    pub fn asteroid_kind(&self) -> AsteroidKind {
        self.asteroid_kind
    }

    pub fn set_asteroid_kind(&mut self, kind: AsteroidKind ){
        self.asteroid_kind=kind;
    }
}
