use crate::game::asset_repository::audio_repository::traits::AudioClips;
use macroquad::audio::{self, Sound};
use strum::EnumIter;

const LOG_PREFIX: &str = "[ship_destruction]";

/// Identifies a named clip for `ShipDestructionSounds`. Each variant
/// pairs with a clip field on `ShipDestructionSounds` below — add both
/// together to register a new destruction sound.
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
pub enum ShipDestructionSound {
    Basic,
}

pub struct ShipDestructionSounds {
    /// Clip for `ShipDestructionSound::Basic`. Add a field here per new variant.
    basic: Option<Sound>,
}

impl ShipDestructionSounds {
    pub fn new() -> Self {
        Self { basic: None }
    }
}

impl AudioClips for ShipDestructionSounds {
    type Kind = ShipDestructionSound;

    /// Exhaustive match: adding a `ShipDestructionSound` variant without
    /// adding it here is a compile error.
    async fn load_clip_for(&mut self, clip_kind: &ShipDestructionSound) {
        match clip_kind {
            ShipDestructionSound::Basic => {
                if self.basic.is_none() {
                    println!("{LOG_PREFIX} loading {clip_kind:?}...");

                    self.basic = Some(
                        audio::load_sound_from_bytes(include_bytes!(
                            "../../../../assets/audio/ship/destruction.wav"
                        ))
                        .await
                        .unwrap_or_else(|_| panic!("{LOG_PREFIX} failed to load {clip_kind:?}")),
                    );

                    println!("{LOG_PREFIX} {clip_kind:?} load complete");
                }
            }
        }
    }

    fn get_clip_for(&self, clip_kind: &ShipDestructionSound) -> Option<&Sound> {
        match clip_kind {
            ShipDestructionSound::Basic => self.basic.as_ref(),
        }
    }
}
