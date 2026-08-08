use macroquad::texture::Texture2D;
use strum::EnumIter;
use crate::game::{
    frames::{load_frames, frame_sequence},
    asset_repository::sprite_repository::traits::{SpriteTextures, Sprite},
};

const LOG_PREFIX: &str = "[asteroid_v1]";

/// Identifies a named texture group for `AsteroidV1`. Each variant pairs
/// with a texture field on `AsteroidV1` below — add both together to
/// register a new asteroid look.
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
pub enum AsteroidV1Textures {
    /// Dark rock veined with glowing molten cracks.
    Molten,
}

pub struct AsteroidV1 {
    /// Frames for `AsteroidV1Textures::Molten`.
    molten_texture: Vec<Texture2D>,
}

impl AsteroidV1 {
    pub fn new() -> Self {
        Self { molten_texture: Vec::new() }
    }
}

impl SpriteTextures for AsteroidV1 {
    type Kind = AsteroidV1Textures;

    /// Exhaustive match: adding an `AsteroidV1Textures` variant without
    /// adding it here is a compile error.
    async fn load_textures_for(&mut self, texture_kind: &AsteroidV1Textures) {
        match texture_kind {
            AsteroidV1Textures::Molten => {
                if self.molten_texture.is_empty() {
                    println!("{LOG_PREFIX} loading {texture_kind:?}...");

                    self.molten_texture = load_frames(
                        frame_sequence!("assets/animations/asteroid/asteroid_", ["0"])
                    ).await;

                    println!("{LOG_PREFIX} {texture_kind:?} load complete");
                }
            }
        }
    }

    fn get_textures_for(&self, texture_kind: &AsteroidV1Textures) -> &Vec<Texture2D> {
        match texture_kind {
            AsteroidV1Textures::Molten => &self.molten_texture,
        }
    }
}

impl Sprite for AsteroidV1 {}
