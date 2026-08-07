pub mod button_v1;
pub mod ship_v1;
pub mod traits;

use button_v1::ButtonV1;
use ship_v1::ShipV1;
use crate::game::asset_repository::{sprite_repository::traits::{Sprite, SpriteTextures}, traits::Preloadable};

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

impl Preloadable for SpriteRepository {
    async fn load_all(&mut self) {
        self.button_v1_sprite.load_all_textures().await;
        self.ship_v1_sprite.load_all_textures().await;
    }
}