use crate::game::asset_repository::sprite_repository::{
    traits::{Sprite, SpriteBounds, SpriteTextures},
    utils::frames::{frame_sequence, load_frames},
};
use macroquad::{
    math::{Rect, Vec2},
    texture::Texture2D,
};
use strum::EnumIter;

pub const SHIP_V1_SPRITE: &str = "ShipV1Sprite";
const LOG_PREFIX: &str = "[ship_v1]";

/// Identifies a named texture group for `ShipV1`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
pub enum ShipV1Textures {
    SentinelAlive,
    SentinelDead,
    VanguardAlive,
    VanguardDead,
    ViperAlive,
    ViperDead,
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
            // 505×492 source frames; drawn ship occupies this sub-rect.
            ShipV1Textures::VanguardAlive => Rect::new(8.0, 22.0, 490.0, 456.0),
            // 505×492 source frames; the debris scatters to the frame edges
            // in the later frames, so the crop is the full canvas.
            ShipV1Textures::VanguardDead => Rect::new(0.0, 0.0, 505.0, 492.0),
            // 382×742 source frames; drawn ship occupies this sub-rect.
            ShipV1Textures::ViperAlive => Rect::new(3.0, 4.0, 373.0, 723.0),
            // 662×922 source frames — each frame was individually generated
            // at its own resolution, then centered onto this shared canvas
            // (required so one crop rect applies correctly to every frame).
            ShipV1Textures::ViperDead => Rect::new(143.0, 24.0, 373.0, 723.0),
        }
    }

    /// Per-variant override of the default `content_bounds`-derived size.
    fn content_size_at_logical_unit_scale(&self) -> Vec2 {
        match self {
            // Vanguard's silhouette is wide and splayed (unlike Sentinel's
            // tall, compact one), so it reads visually smaller than Sentinel
            // when fit into the same box. Draw it 40% larger to compensate.
            ShipV1Textures::VanguardAlive => self.content_bounds().size() * 1.4,
            // Viper reads small next to the other ships at the same fit box;
            // draw it 20% larger.
            ShipV1Textures::ViperAlive => self.content_bounds().size() * 1.2,
            _ => self.content_bounds().size(),
        }
    }
}

pub struct ShipV1 {
    /// Frames for `ShipV1Textures::Sentinel`
    sentinel_alive_texture: Vec<Texture2D>,
    sentinel_dead_texture: Vec<Texture2D>,
    /// Frames for `ShipV1Textures::VanguardAlive`
    vanguard_alive_texture: Vec<Texture2D>,
    vanguard_dead_texture: Vec<Texture2D>,
    viper_alive_texture: Vec<Texture2D>,
    viper_dead_texture: Vec<Texture2D>,
    // Add field pair here per new variant in ShipV1Textures.
}

impl Default for ShipV1 {
    fn default() -> Self {
        Self::new()
    }
}

impl ShipV1 {
    pub fn new() -> Self {
        Self {
            sentinel_alive_texture: Vec::new(),
            sentinel_dead_texture: Vec::new(),
            vanguard_alive_texture: Vec::new(),
            vanguard_dead_texture: Vec::new(),
            viper_alive_texture: Vec::new(),
            viper_dead_texture: Vec::new(),
        }
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

                    self.sentinel_alive_texture = load_frames(frame_sequence!(
                        "assets/animations/ships/sentinel/alive/sentinel_",
                        ["00", "01", "02", "03", "04", "05", "06", "07"]
                    ))
                    .await;

                    println!("{LOG_PREFIX} {texture_kind:?} load complete");
                }
            }

            ShipV1Textures::SentinelDead => {
                if self.sentinel_dead_texture.is_empty() {
                    println!("{LOG_PREFIX} loading {texture_kind:?}...");

                    self.sentinel_dead_texture = load_frames(frame_sequence!(
                        "assets/animations/ships/sentinel/dead/sentinel_dead_",
                        ["00", "01", "02", "03", "04"]
                    ))
                    .await;

                    println!("{LOG_PREFIX} {texture_kind:?} load complete");
                }
            }

            ShipV1Textures::VanguardAlive => {
                if self.vanguard_alive_texture.is_empty() {
                    println!("{LOG_PREFIX} loading {texture_kind:?}...");

                    self.vanguard_alive_texture = load_frames(frame_sequence!(
                        "assets/animations/ships/vanguard/alive/vanguard_",
                        ["00", "01", "02"]
                    ))
                    .await;

                    println!("{LOG_PREFIX} {texture_kind:?} load complete");
                }
            }

            ShipV1Textures::VanguardDead => {
                if self.vanguard_dead_texture.is_empty() {
                    println!("{LOG_PREFIX} loading {texture_kind:?}...");

                    self.vanguard_dead_texture = load_frames(frame_sequence!(
                        "assets/animations/ships/vanguard/dead/vanguard_dead_",
                        ["00", "01", "02", "03", "04"]
                    ))
                    .await;

                    println!("{LOG_PREFIX} {texture_kind:?} load complete");
                }
            }

            ShipV1Textures::ViperAlive => {
                if self.viper_alive_texture.is_empty() {
                    println!("{LOG_PREFIX} loading {texture_kind:?}...");

                    self.viper_alive_texture = load_frames(frame_sequence!(
                        "assets/animations/ships/viper/alive/viper_",
                        ["00", "01", "02", "03"]
                    ))
                    .await;

                    println!("{LOG_PREFIX} {texture_kind:?} load complete");
                }
            }

            ShipV1Textures::ViperDead => {
                if self.viper_dead_texture.is_empty() {
                    println!("{LOG_PREFIX} loading {texture_kind:?}...");

                    self.viper_dead_texture = load_frames(frame_sequence!(
                        "assets/animations/ships/viper/dead/viper_dead_",
                        ["00", "01", "02", "03", "04"]
                    ))
                    .await;

                    println!("{LOG_PREFIX} {texture_kind:?} load complete");
                }
            }
        }
    }

    fn get_textures_for(&self, texture_kind: &Self::Kind) -> &Vec<Texture2D> {
        match texture_kind {
            ShipV1Textures::SentinelAlive => &self.sentinel_alive_texture,
            ShipV1Textures::SentinelDead => &self.sentinel_dead_texture,
            ShipV1Textures::VanguardAlive => &self.vanguard_alive_texture,
            ShipV1Textures::VanguardDead => &self.vanguard_dead_texture,
            ShipV1Textures::ViperAlive => &self.viper_alive_texture,
            ShipV1Textures::ViperDead => &self.viper_dead_texture,
        }
    }
}

impl Sprite for ShipV1 {}
