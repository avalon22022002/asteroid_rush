pub mod asteroid_v1;
pub mod banner_v1;
pub mod button_v1;
pub mod ship_v1;
pub mod traits;

use std::sync::OnceLock;

use futures::executor;

pub use asteroid_v1::*;
pub use banner_v1::*;
pub use button_v1::*;
pub use ship_v1::*;

use crate::game::asset_repository::{
    sprite_repository::traits::SpriteTextures,
    traits::{Preloadable, Singleton},
};

/// # Example
///
/// ```no_run
/// use asteroid_rush::game::asset_repository::{
///     traits::Singleton,
///     sprite_repository::{SpriteRepository, traits::SpriteTextures, button_v1::ButtonV1Textures},
/// };
///
/// // get_instance builds (and loads) the repository on first call, so it's
/// // already fully loaded here — no separate load_all step needed.
/// let repo = SpriteRepository::get_instance();
/// let textures = repo.button_v1_sprite.get_textures_for(&ButtonV1Textures::BasicScifiV1);
/// ```

pub struct SpriteRepository {
    pub asteroid_v1_sprite: AsteroidV1,
    pub banner_v1_sprite: BannerV1,
    pub button_v1_sprite: ButtonV1,
    pub ship_v1_sprite: ShipV1,
}

impl SpriteRepository {
    pub fn new() -> Self {
        Self {
            asteroid_v1_sprite: AsteroidV1::new(),
            banner_v1_sprite: BannerV1::new(),
            button_v1_sprite: ButtonV1::new(),
            ship_v1_sprite: ShipV1::new()
        }
    }
}

static INSTANCE: OnceLock<SpriteRepository> = OnceLock::new();

impl Singleton for SpriteRepository {
    fn storage() -> &'static OnceLock<Self> {
        &INSTANCE
    }

    /// Builds a fully loaded `SpriteRepository`. `block_on`'s `load_all` to
    /// stay sync, same tradeoff (and reasoning) as `AudioRepository::new`.
    fn new() -> Self {
        let mut repo = SpriteRepository::new();
        executor::block_on(repo.load_all());
        repo
    }
}

impl Preloadable for SpriteRepository {
    /// Loads every sprite type concurrently (`futures::join!`) rather than
    /// one after another, so the total wait is the slowest single load, not
    /// their sum.
    async fn load_all(&mut self) {
        futures::join!(
            self.asteroid_v1_sprite.load_all_textures(),
            self.banner_v1_sprite.load_all_textures(),
            self.button_v1_sprite.load_all_textures(),
            self.ship_v1_sprite.load_all_textures(),
        );
    }
}
