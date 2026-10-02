use macroquad::audio::{self, Sound};
use strum::EnumIter;
use crate::game::asset_repository::audio_repository::traits::AudioClips;

const LOG_PREFIX: &str = "[ship_damage]";

/// Identifies a named clip for `ShipDamageSounds`. Each variant pairs with
/// a clip field on `ShipDamageSounds` below — add both together to
/// register a new damage-taken sound.
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
pub enum ShipDamageSound {
    Basic,
}

pub struct ShipDamageSounds {
    /// Clip for `ShipDamageSound::Basic`. Add a field here per new variant.
    basic: Option<Sound>,
}

impl ShipDamageSounds {
    pub fn new() -> Self {
        Self { basic: None }
    }
}

impl AudioClips for ShipDamageSounds {
    type Kind = ShipDamageSound;

    /// Exhaustive match: adding a `ShipDamageSound` variant without adding
    /// it here is a compile error.
    async fn load_clip_for(&mut self, clip_kind: &ShipDamageSound) {
        match clip_kind {
            ShipDamageSound::Basic => {
                if self.basic.is_none() {
                    println!("{LOG_PREFIX} loading {clip_kind:?}...");

                    self.basic = Some(
                        audio::load_sound_from_bytes(include_bytes!(
                            "../../../../assets/audio/ship/damage.wav"
                        ))
                        .await
                        .unwrap_or_else(|_| panic!("{LOG_PREFIX} failed to load {clip_kind:?}")),
                    );

                    println!("{LOG_PREFIX} {clip_kind:?} load complete");
                }
            }
        }
    }

    fn get_clip_for(&self, clip_kind: &ShipDamageSound) -> Option<&Sound> {
        match clip_kind {
            ShipDamageSound::Basic => self.basic.as_ref(),
        }
    }
}
