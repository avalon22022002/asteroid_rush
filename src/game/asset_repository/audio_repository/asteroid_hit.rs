use crate::game::asset_repository::audio_repository::traits::AudioClips;
use macroquad::audio::{self, Sound};
use strum::EnumIter;

const LOG_PREFIX: &str = "[asteroid_hit]";

/// Identifies a named clip for `AsteroidHitSounds`. Each variant pairs
/// with a clip field on `AsteroidHitSounds` below — add both together to
/// register a new hit sound.
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
pub enum AsteroidHitSound {
    Tick,
}

pub struct AsteroidHitSounds {
    /// Clip for `AsteroidHitSound::Tick`. Add a field here per new variant.
    tick: Option<Sound>,
}

impl AsteroidHitSounds {
    pub fn new() -> Self {
        Self { tick: None }
    }
}

impl AudioClips for AsteroidHitSounds {
    type Kind = AsteroidHitSound;

    /// Exhaustive match: adding an `AsteroidHitSound` variant without
    /// adding it here is a compile error.
    async fn load_clip_for(&mut self, clip_kind: &AsteroidHitSound) {
        match clip_kind {
            AsteroidHitSound::Tick => {
                if self.tick.is_none() {
                    println!("{LOG_PREFIX} loading {clip_kind:?}...");

                    self.tick = Some(
                        audio::load_sound_from_bytes(include_bytes!(
                            "../../../../assets/audio/asteroid/hit.wav"
                        ))
                        .await
                        .unwrap_or_else(|_| panic!("{LOG_PREFIX} failed to load {clip_kind:?}")),
                    );

                    println!("{LOG_PREFIX} {clip_kind:?} load complete");
                }
            }
        }
    }

    fn get_clip_for(&self, clip_kind: &AsteroidHitSound) -> Option<&Sound> {
        match clip_kind {
            AsteroidHitSound::Tick => self.tick.as_ref(),
        }
    }
}
