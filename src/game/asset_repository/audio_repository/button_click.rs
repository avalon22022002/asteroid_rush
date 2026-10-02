use crate::game::asset_repository::audio_repository::traits::AudioClips;
use macroquad::audio::{self, Sound};
use strum::EnumIter;

const LOG_PREFIX: &str = "[button_click]";

/// Identifies a named clip for `ButtonClickSounds`. Each variant pairs with
/// a clip field on `ButtonClickSounds` below — add both together to
/// register a new button-click sound.
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
pub enum ButtonClickSound {
    Basic,
}

pub struct ButtonClickSounds {
    /// Clip for `ButtonClickSound::Basic`. Add a field here per new variant.
    basic_click: Option<Sound>,
}

impl ButtonClickSounds {
    pub fn new() -> Self {
        Self { basic_click: None }
    }
}

impl AudioClips for ButtonClickSounds {
    type Kind = ButtonClickSound;

    /// Exhaustive match: adding a `ButtonClickSound` variant without adding
    /// it here is a compile error.
    async fn load_clip_for(&mut self, clip_kind: &ButtonClickSound) {
        match clip_kind {
            ButtonClickSound::Basic => {
                if self.basic_click.is_none() {
                    println!("{LOG_PREFIX} loading {clip_kind:?}...");

                    self.basic_click = Some(
                        audio::load_sound_from_bytes(include_bytes!(
                            "../../../../assets/audio/ui/click.wav"
                        ))
                        .await
                        .unwrap_or_else(|_| panic!("{LOG_PREFIX} failed to load {clip_kind:?}")),
                    );

                    println!("{LOG_PREFIX} {clip_kind:?} load complete");
                }
            }
        }
    }

    fn get_clip_for(&self, clip_kind: &ButtonClickSound) -> Option<&Sound> {
        match clip_kind {
            ButtonClickSound::Basic => self.basic_click.as_ref(),
        }
    }
}
