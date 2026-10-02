use crate::game::asset_repository::audio_repository::traits::AudioClips;
use macroquad::audio::{self, Sound};
use strum::EnumIter;

const LOG_PREFIX: &str = "[gun_fire]";

/// Identifies a named clip for `GunFireSounds`. Each variant pairs with
/// a clip field on `GunFireSounds` below — add both together to
/// register a new gun-fire sound.
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
pub enum GunFireSound {
    Laser,
}

pub struct GunFireSounds {
    /// Clip for `GunFireSound::Laser`. Add a field here per new variant.
    laser: Option<Sound>,
}

impl GunFireSounds {
    pub fn new() -> Self {
        Self { laser: None }
    }
}

impl AudioClips for GunFireSounds {
    type Kind = GunFireSound;

    /// Exhaustive match: adding a `GunFireSound` variant without adding
    /// it here is a compile error.
    async fn load_clip_for(&mut self, clip_kind: &GunFireSound) {
        match clip_kind {
            GunFireSound::Laser => {
                if self.laser.is_none() {
                    println!("{LOG_PREFIX} loading {clip_kind:?}...");

                    self.laser = Some(
                        audio::load_sound_from_bytes(include_bytes!(
                            "../../../../assets/audio/weapons/laser.wav"
                        ))
                        .await
                        .unwrap_or_else(|_| panic!("{LOG_PREFIX} failed to load {clip_kind:?}")),
                    );

                    println!("{LOG_PREFIX} {clip_kind:?} load complete");
                }
            }
        }
    }

    fn get_clip_for(&self, clip_kind: &GunFireSound) -> Option<&Sound> {
        match clip_kind {
            GunFireSound::Laser => self.laser.as_ref(),
        }
    }
}
