pub mod button_v1;
pub mod ship_v1;
pub mod traits;

use std::sync::OnceLock;

use futures::executor;

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
/// let textures = repo.button_v1_sprite.get_textures_for(ButtonV1Textures::BasicScifiV1);
/// ```

pub struct SpriteRepository {
    pub button_v1_sprite: ButtonV1,
    pub ship_v1_sprite: ShipV1,
}

impl SpriteRepository {
    pub fn new() -> Self {
        Self {
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
    async fn load_all(&mut self) {
        self.button_v1_sprite.load_all_textures().await;
        self.ship_v1_sprite.load_all_textures().await;
    }
}
