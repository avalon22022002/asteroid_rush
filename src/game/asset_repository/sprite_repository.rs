pub mod button_v1;
pub mod ship_v1;
pub mod traits;

use button_v1::ButtonV1;
use ship_v1::ShipV1;
use crate::game::asset_repository::{self, sprite_repository::traits::SpriteTextures};

pub enum Sprites{
    ShipV1
}
pub struct SpriteRepository {
    button_v1_sprite: ButtonV1,
    ship_v1_sprite: ShipV1
}

impl SpriteRepository {
    fn new()-> Self{
        Self { button_v1_sprite: ButtonV1::new(), ship_v1_sprite: ShipV1::new() }
    }
}
