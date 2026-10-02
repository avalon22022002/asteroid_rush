use crate::game::asset_repository::audio_repository::traits::AudioClips;
use macroquad::audio::{self, Sound};
use strum::EnumIter;

const LOG_PREFIX: &str = "[button_hover]";

/// Identifies a named clip for `ButtonHoverSounds`. Each variant pairs with
/// a clip field on `ButtonHoverSounds` below — add both together to
/// register a new button-hover sound.
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
pub enum ButtonHoverSound {
    Basic,
}

pub struct ButtonHoverSounds {
    /// Clip for `ButtonHoverSound::Basic`. Add a field here per new variant.
    basic_hover: Option<Sound>,
}

impl Default for ButtonHoverSounds {
    fn default() -> Self {
        Self::new()
    }
}

impl ButtonHoverSounds {
    pub fn new() -> Self {
        Self { basic_hover: None }
    }
}

impl AudioClips for ButtonHoverSounds {
    type Kind = ButtonHoverSound;

    /// Exhaustive match: adding a `ButtonHoverSound` variant without adding
    /// it here is a compile error.
    async fn load_clip_for(&mut self, clip_kind: &ButtonHoverSound) {
        match clip_kind {
            ButtonHoverSound::Basic => {
                if self.basic_hover.is_none() {
                    println!("{LOG_PREFIX} loading {clip_kind:?}...");

                    self.basic_hover = Some(
                        audio::load_sound_from_bytes(include_bytes!(
                            "../../../../assets/audio/ui/hover.wav"
                        ))
                        .await
                        .unwrap_or_else(|_| panic!("{LOG_PREFIX} failed to load {clip_kind:?}")),
                    );

                    println!("{LOG_PREFIX} {clip_kind:?} load complete");
                }
            }
        }
    }

    fn get_clip_for(&self, clip_kind: &ButtonHoverSound) -> Option<&Sound> {
        match clip_kind {
            ButtonHoverSound::Basic => self.basic_hover.as_ref(),
        }
    }
}
