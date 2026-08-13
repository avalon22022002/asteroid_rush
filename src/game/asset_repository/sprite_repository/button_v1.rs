use macroquad::{math::Rect, texture::Texture2D};
use strum::EnumIter;
use crate::game::{
    frames::{load_frames, frame_sequence},
    asset_repository::sprite_repository::traits::{SpriteTextures,Sprite,SpriteBounds}
};

const LOG_PREFIX: &str = "[button_v1]";

/// Identifies a named texture group for `ButtonV1`. Each variant pairs with
/// a texture field on `ButtonV1` below — add both together to register a
/// new button-v1 look.
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
pub enum ButtonV1Textures {
    BasicScifiV1,
}

impl SpriteBounds for ButtonV1Textures {
    /// The button art (the button's texture PNG) has empty transparent space
    /// around the drawn button. This returns the rect covering just the drawn
    /// button, so callers can sample that and leave the empty border out.
    fn content_bounds(&self) -> Rect {
        match self {
            // BasicScifiV1 is a 690×362 PNG; the drawn button sits in this sub-rect.
            ButtonV1Textures::BasicScifiV1 => Rect::new(54.0, 68.0, 589.0, 231.0),
        }
    }
}

pub struct ButtonV1 {
    /// Frames for `ButtonV1Textures::BasicScifiV1`. Add a field here per new variant.
    scifi_v1_texture: Vec<Texture2D>,
}

impl ButtonV1 {
    pub fn new() -> Self {
        Self { scifi_v1_texture: Vec::new() }
    }
}

impl SpriteTextures for ButtonV1 {
    type Kind = ButtonV1Textures;

    /// Exhaustive match: adding a `ButtonV1Textures` variant without adding
    /// it here is a compile error.
    async fn load_textures_for(&mut self, texture_kind: &ButtonV1Textures) {
        match texture_kind {
            ButtonV1Textures::BasicScifiV1 => {
                if self.scifi_v1_texture.is_empty() {
                    println!("{LOG_PREFIX} loading {texture_kind:?}...");

                    self.scifi_v1_texture = load_frames(
                        frame_sequence!("assets/ui/button/button-background_", ["00"])
                    ).await;

                    println!("{LOG_PREFIX} {texture_kind:?} load complete");
                }
            }
        }
    }

    fn get_textures_for(&self, texture_kind: &ButtonV1Textures) -> &Vec<Texture2D> {
        match texture_kind {
            ButtonV1Textures::BasicScifiV1 => &self.scifi_v1_texture,
        }
    }
}

impl Sprite for ButtonV1 {}