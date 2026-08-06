use macroquad::texture::Texture2D;
use strum::{EnumIter, IntoEnumIterator};
use crate::game::frames::{load_frames, frame_sequence};

const LOG_PREFIX: &str = "[button_v1]";

/// Identifies a named texture group for `ButtonV1`. Each variant pairs with
/// a texture field on `ButtonV1` below — add both together to register a
/// new button-v1 look.
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
pub enum ButtonV1Textures {
    BasicScifiV1,
}

pub struct ButtonV1 {
    /// Frames for `ButtonV1Textures::BasicScifiV1`. Add a field here per new variant.
    scifi_v1_texture: Vec<Texture2D>,
}

impl ButtonV1 {
    fn new() -> Self {
        Self { scifi_v1_texture: Vec::new() }
    }

    /// Returns the frames for `texture_kind`, loading them on first use.
    /// Exhaustive match: adding a `ButtonV1Textures` variant without adding
    /// it here is a compile error.
    async fn load_textures_for(&mut self, texture_kind: ButtonV1Textures) -> &Vec<Texture2D> {
        match texture_kind {
            ButtonV1Textures::BasicScifiV1 => {
                if self.scifi_v1_texture.is_empty() {
                    println!("{LOG_PREFIX} loading {texture_kind:?}...");

                    self.scifi_v1_texture = load_frames(
                        frame_sequence!("assets/ui/button/button-background_", ["00"])
                    ).await;
                    
                    println!("{LOG_PREFIX} {texture_kind:?} load complete");
                }
                &self.scifi_v1_texture
            }
        }
    }

    /// Eagerly loads every `ButtonV1Textures` kind. Call once at game init,
    /// same as `AssetRepository::load_all`.
    async fn load_all_textures(&mut self) {
        for texture_kind in ButtonV1Textures::iter() {
            self.load_textures_for(texture_kind).await;
        }
    }
}
