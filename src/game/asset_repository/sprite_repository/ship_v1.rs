use macroquad::{math::Rect, texture::Texture2D};
use strum::EnumIter;
use crate::game::asset_repository::sprite_repository::{
    utils::frames::{load_frames, frame_sequence},
    traits::{SpriteTextures,Sprite,SpriteBounds},
};

pub const SHIP_V1_SPRITE: &str="ShipV1Sprite";
const LOG_PREFIX: &str = "[ship_v1]";

/// Identifies a named texture group for `ShipV1`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
pub enum ShipV1Textures {
    SentinelAlive,
    SentinelDead,
}

impl SpriteBounds for ShipV1Textures {
    /// The ship art sits in a large transparent frame. This is the union of
    /// every animation frame's opaque box (measured from the source PNGs), so
    /// the crop holds steady across the whole loop instead of jittering as the
    /// thruster flames change height.
    fn content_bounds(&self) -> Rect {
        match self {
            // 488×620 source frames; drawn ship occupies this sub-rect.
            ShipV1Textures::SentinelAlive => Rect::new(83.0, 23.0, 363.0, 462.0),
            // 252×264 source frames; drawn explosion occupies this sub-rect.
            ShipV1Textures::SentinelDead => Rect::new(26.0, 0.0, 207.0, 264.0),
        }
    }
}

pub struct ShipV1 {
    /// Frames for `ShipV1Textures::Sentinel`
    sentinel_alive_texture: Vec<Texture2D>,
    sentinel_dead_texture: Vec<Texture2D>,

    // Add field pair here per new variant in ShipV1Textures.
}

impl ShipV1 {
    pub fn new() -> Self {
        Self { sentinel_alive_texture: Vec::new(), sentinel_dead_texture: Vec::new() }
    }
}

impl SpriteTextures for ShipV1 {
    type Kind = ShipV1Textures;

    async fn load_textures_for(&mut self, texture_kind: &ShipV1Textures) {
        
        // Perform Exhaustive match.
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

            ShipV1Textures::SentinelDead => {
                if self.sentinel_dead_texture.is_empty() {
                    println!("{LOG_PREFIX} loading {texture_kind:?}...");

                    self.sentinel_dead_texture = load_frames(
                        frame_sequence!("assets/animations/ships/sentinel/dead/sentinel_dead_", ["00","01","02","03","04"])
                    ).await;

                    println!("{LOG_PREFIX} {texture_kind:?} load complete");
                }
            }
        }
    }

    fn get_textures_for(&self, texture_kind: &Self::Kind) -> &Vec<Texture2D> {
        match texture_kind {
            ShipV1Textures::SentinelAlive => &self.sentinel_alive_texture,
            ShipV1Textures::SentinelDead => &self.sentinel_dead_texture,
        }
    }
}

impl Sprite for ShipV1 {}