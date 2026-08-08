use macroquad::texture::Texture2D;
use strum::EnumIter;
use crate::game::{
    frames::{load_frames, frame_sequence},
    asset_repository::sprite_repository::traits::{SpriteTextures, Sprite},
};

const LOG_PREFIX: &str = "[banner_v1]";

/// Identifies a named texture group for `BannerV1`. Each variant pairs with
/// a texture field on `BannerV1` below — add both together to register a
/// new banner.
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
pub enum BannerV1Textures {
    HomePageMain,
    LevelSelectionPageMain,
}

pub struct BannerV1 {
    /// Frames for `BannerV1Textures::HomePageMain`.
    home_page_main_texture: Vec<Texture2D>,
    /// Frames for `BannerV1Textures::LevelSelectionPageMain`.
    level_selection_page_main_texture: Vec<Texture2D>,
}

impl BannerV1 {
    pub fn new() -> Self {
        Self {
            home_page_main_texture: Vec::new(),
            level_selection_page_main_texture: Vec::new(),
        }
    }
}

impl SpriteTextures for BannerV1 {
    type Kind = BannerV1Textures;

    /// Exhaustive match: adding a `BannerV1Textures` variant without adding
    /// it here is a compile error.
    async fn load_textures_for(&mut self, texture_kind: &BannerV1Textures) {
        match texture_kind {
            BannerV1Textures::HomePageMain => {
                if self.home_page_main_texture.is_empty() {
                    println!("{LOG_PREFIX} loading {texture_kind:?}...");

                    self.home_page_main_texture = load_frames(
                        frame_sequence!("assets/ui/text/home_page_title", [""])
                    ).await;

                    println!("{LOG_PREFIX} {texture_kind:?} load complete");
                }
            }
            BannerV1Textures::LevelSelectionPageMain => {
                if self.level_selection_page_main_texture.is_empty() {
                    println!("{LOG_PREFIX} loading {texture_kind:?}...");

                    self.level_selection_page_main_texture = load_frames(
                        frame_sequence!("assets/ui/text/level_selection_page_title", [""])
                    ).await;

                    println!("{LOG_PREFIX} {texture_kind:?} load complete");
                }
            }
        }
    }

    fn get_textures_for(&self, texture_kind: &BannerV1Textures) -> &Vec<Texture2D> {
        match texture_kind {
            BannerV1Textures::HomePageMain => &self.home_page_main_texture,
            BannerV1Textures::LevelSelectionPageMain => &self.level_selection_page_main_texture,
        }
    }
}

impl Sprite for BannerV1 {}
