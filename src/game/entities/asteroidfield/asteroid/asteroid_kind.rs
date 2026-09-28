use crate::game::{
    asset_repository::sprite_repository::AsteroidV1Textures,
    entities::asteroidfield::asteroid::asteroid_stats::AsteroidStats,
    game_config::GameLevel,
    utils::MinMax,
};

/// Identifies which asteroid texture to draw. Add a variant here (and a
/// case in `AsteroidKind::texture_kind`) to register a new asteroid look.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AsteroidKind {
    /// Dark rock veined with glowing molten cracks.
    MoltenDarkAsteroid,
    /// Dark rock wreathed in cold blue flame, veined with purple energy.
    CryoflareAsteroid,
}

impl AsteroidKind {
    pub fn asteroid_kind_from_level(level: &GameLevel) -> AsteroidKind {
        match level {
            GameLevel::Level1 => AsteroidKind::MoltenDarkAsteroid,
            GameLevel::Level2 => AsteroidKind::CryoflareAsteroid,
            GameLevel::Level3 => AsteroidKind::CryoflareAsteroid,
        }
    }

    /// This kind's texture group in `SpriteRepository`.
    pub fn texture_kind(&self) -> AsteroidV1Textures {
        match self {
            AsteroidKind::MoltenDarkAsteroid => AsteroidV1Textures::MoltenDark,
            AsteroidKind::CryoflareAsteroid => AsteroidV1Textures::Cryoflare,
        }
    }

    pub fn stat_range(&self) -> MinMax<AsteroidStats>{
        AsteroidStats::range_for(*self)
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Self::MoltenDarkAsteroid => "Molten Dark Asteroid",
            Self::CryoflareAsteroid => "Cryoflare Asteroid",
        }
    }

    pub fn difficulty_label(&self) -> &'static str {
        match self {
            Self::MoltenDarkAsteroid => "Beginner Level Asteroid",
            Self::CryoflareAsteroid => "Intermediate Level Asteroid",
        }
    }

    /// Stats shown on the level-select preview card, as label/value pairs.
    pub fn preview_stats(&self) -> Vec<(String, String)> {
        let max_stats = self.stat_range().max;
        vec![
            ("Max speed".to_string(), format!("{}", max_stats.speed())),
            ("Max rotation speed".to_string(), format!("{}", max_stats.rotation_speed())),
            ("Max health".to_string(), format!("{}", max_stats.max_health())),
            ("Max collision damage".to_string(), format!("{}", max_stats.damage_on_collision())),
        ]
    }
}
