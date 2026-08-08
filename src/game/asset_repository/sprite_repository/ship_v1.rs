use macroquad::texture::Texture2D;
use strum::EnumIter;
use crate::game::{
    frames::{load_frames, frame_sequence},
    asset_repository::sprite_repository::traits::{SpriteTextures,Sprite},
};

pub const SHIP_V1_SPRITE: &str="ShipV1Sprite";
const LOG_PREFIX: &str = "[ship_v1]";

/// Identifies a named texture group for `ShipV1`. Each variant pairs with
/// a texture field on `ShipV1` below — add both together to register a
/// new ship-v1 look.
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
pub enum ShipV1Textures {
    SentinelAlive,
}

pub struct ShipV1 {
    /// Frames for `ShipV1Textures::Sentinel`. Add a field here per new variant.
    sentinel_alive_texture: Vec<Texture2D>,
    sentinel_dead_texture: Vec<Texture2D>,
}

impl ShipV1 {
    pub fn new() -> Self {
        Self { sentinel_alive_texture: Vec::new(), sentinel_dead_texture: Vec::new() }
    }
}

impl SpriteTextures for ShipV1 {
    type Kind = ShipV1Textures;

    /// Exhaustive match: adding a `ShipV1Textures` variant without adding it
    /// here is a compile error.
    async fn load_textures_for(&mut self, texture_kind: &ShipV1Textures) {
        match texture_kind {
            ShipV1Textures::SentinelAlive => {
                if self.sentinel_alive_texture.is_empty() {
                    println!("{LOG_PREFIX} loading {texture_kind:?}...");

                    self.sentinel_alive_texture = load_frames(
                        frame_sequence!("assets/animations/ships/sentinel/alive/sentinel_", ["00","01","02","03", "04","05","06","07"])
                    ).await;

                    println!("{LOG_PREFIX} {texture_kind:?} load complete");
                }
            }
        }
    }

    fn get_textures_for(&self, texture_kind: &Self::Kind) -> &Vec<Texture2D> {
        match texture_kind {
            ShipV1Textures::SentinelAlive => &self.sentinel_alive_texture
        }
    }
}

impl Sprite for ShipV1 {}