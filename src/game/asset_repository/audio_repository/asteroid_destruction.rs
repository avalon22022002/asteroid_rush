use crate::game::asset_repository::audio_repository::traits::AudioClips;
use macroquad::audio::{self, Sound};
use strum::EnumIter;

const LOG_PREFIX: &str = "[asteroid_destruction]";

/// Identifies a named clip for `AsteroidDestructionSounds`. Each variant
/// pairs with a clip field on `AsteroidDestructionSounds` below — add both
/// together to register a new destruction sound.
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
pub enum AsteroidDestructionSound {
    Retro,
}

pub struct AsteroidDestructionSounds {
    /// Clip for `AsteroidDestructionSound::Retro`. Add a field here per new variant.
    retro: Option<Sound>,
}

impl AsteroidDestructionSounds {
    pub fn new() -> Self {
        Self { retro: None }
    }
}

impl AudioClips for AsteroidDestructionSounds {
    type Kind = AsteroidDestructionSound;

    /// Exhaustive match: adding an `AsteroidDestructionSound` variant
    /// without adding it here is a compile error.
    async fn load_clip_for(&mut self, clip_kind: &AsteroidDestructionSound) {
        match clip_kind {
            AsteroidDestructionSound::Retro => {
                if self.retro.is_none() {
                    println!("{LOG_PREFIX} loading {clip_kind:?}...");

                    self.retro = Some(
                        audio::load_sound_from_bytes(include_bytes!(
                            "../../../../assets/audio/asteroid/destruction.wav"
                        ))
                        .await
                        .unwrap_or_else(|_| panic!("{LOG_PREFIX} failed to load {clip_kind:?}")),
                    );

                    println!("{LOG_PREFIX} {clip_kind:?} load complete");
                }
            }
        }
    }

    fn get_clip_for(&self, clip_kind: &AsteroidDestructionSound) -> Option<&Sound> {
        match clip_kind {
            AsteroidDestructionSound::Retro => self.retro.as_ref(),
        }
    }
}
