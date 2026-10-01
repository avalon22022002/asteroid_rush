use macroquad::texture::Texture2D;
use strum::EnumIter;
use crate::game::asset_repository::sprite_repository::{
    utils::frames::{load_frames, frame_sequence},
    traits::{SpriteTextures, Sprite, SpriteBounds},
};
use macroquad::prelude::*;

const LOG_PREFIX: &str = "[asteroid_v1]";

/// Identifies a named texture group for `AsteroidV1`. Each variant pairs
/// with a texture field on `AsteroidV1` below — add both together to
/// register a new asteroid look.
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
pub enum AsteroidV1Textures {
    /// Dark rock veined with glowing molten cracks.
    MoltenDark,
    /// Dark rock wreathed in cold blue flame, veined with purple energy.
    Cryoflare,
    /// Grey rock studded with jagged blue crystal shards, veined with electric cracks.
    Crystalshard,
}

impl SpriteBounds for AsteroidV1Textures {
    /// Bounds cover the visible asteroid and its small floating debris,
    /// providing a stable crop for the sprite without including excess
    /// transparent padding around the artwork.
    fn content_bounds(&self) -> Rect {
        match self {
            // Art sits within a large transparent 3072×3072 frame.
            AsteroidV1Textures::MoltenDark => Rect::new(298.0, 325.0, 2470.0, 2391.0),
            // Art sits within a 500×500 frame.
            AsteroidV1Textures::Cryoflare => Rect::new(22.0, 41.0, 454.0, 421.0),
            // Art sits within a 500×500 frame.
            AsteroidV1Textures::Crystalshard => Rect::new(15.0, 26.0, 472.0, 465.0),
        }
    }

    /// Per-variant override of the default `content_bounds`-derived size,
    /// for variants whose crop is too large to draw as-is.
    fn content_size_at_logical_unit_scale(&self) -> Vec2 {
        match self {
            // The falling-field asteroid should draw much smaller than its
            // full crop, shrink it by 97%.
            AsteroidV1Textures::MoltenDark => self.content_bounds().size() * 0.03,
            // Source canvas is much smaller than MoltenDark's (500×500 vs
            // 3072×3072), shrink it to match its on-screen size.
            AsteroidV1Textures::Cryoflare => self.content_bounds().size() * 0.1,
            // Same 500×500-canvas scale as Cryoflare.
            AsteroidV1Textures::Crystalshard => self.content_bounds().size() * 0.1,
        }
    }
}

pub struct AsteroidV1 {
    /// Frames for `AsteroidV1Textures::MoltenDark`.
    molten_dark_texture: Vec<Texture2D>,
    /// Frames for `AsteroidV1Textures::Cryoflare`.
    cryoflare_texture: Vec<Texture2D>,
    /// Frames for `AsteroidV1Textures::Crystalshard`.
    crystalshard_texture: Vec<Texture2D>,
}

impl AsteroidV1 {
    pub fn new() -> Self {
        Self { molten_dark_texture: Vec::new(), cryoflare_texture: Vec::new(), crystalshard_texture: Vec::new() }
    }
}

impl SpriteTextures for AsteroidV1 {
    type Kind = AsteroidV1Textures;

    /// Exhaustive match: adding an `AsteroidV1Textures` variant without
    /// adding it here is a compile error.
    async fn load_textures_for(&mut self, texture_kind: &AsteroidV1Textures) {
        match texture_kind {
            AsteroidV1Textures::MoltenDark => {
                if self.molten_dark_texture.is_empty() {
                    println!("{LOG_PREFIX} loading {texture_kind:?}...");

                    self.molten_dark_texture = load_frames(
                        frame_sequence!("assets/animations/asteroid/asteroid_", ["0"])
                    ).await;

                    println!("{LOG_PREFIX} {texture_kind:?} load complete");
                }
            }

            AsteroidV1Textures::Cryoflare => {
                if self.cryoflare_texture.is_empty() {
                    println!("{LOG_PREFIX} loading {texture_kind:?}...");

                    self.cryoflare_texture = load_frames(
                        frame_sequence!("assets/animations/asteroid/asteroid_", ["1"])
                    ).await;

                    println!("{LOG_PREFIX} {texture_kind:?} load complete");
                }
            }

            AsteroidV1Textures::Crystalshard => {
                if self.crystalshard_texture.is_empty() {
                    println!("{LOG_PREFIX} loading {texture_kind:?}...");

                    self.crystalshard_texture = load_frames(
                        frame_sequence!("assets/animations/asteroid/asteroid_", ["3"])
                    ).await;

                    println!("{LOG_PREFIX} {texture_kind:?} load complete");
                }
            }
        }
    }

    fn get_textures_for(&self, texture_kind: &AsteroidV1Textures) -> &Vec<Texture2D> {
        match texture_kind {
            AsteroidV1Textures::MoltenDark => &self.molten_dark_texture,
            AsteroidV1Textures::Cryoflare => &self.cryoflare_texture,
            AsteroidV1Textures::Crystalshard => &self.crystalshard_texture,
        }
    }
}

impl Sprite for AsteroidV1 {}
