use crate::game::asset_repository::audio_repository::traits::AudioClips;
use macroquad::audio::{self, Sound};
use strum::EnumIter;

const LOG_PREFIX: &str = "[game_outcome]";

/// Identifies a named clip for `GameOutcomeSounds`. Each variant pairs
/// with a clip field on `GameOutcomeSounds` below — add both together to
/// register a new outcome stinger.
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
pub enum GameOutcomeSound {
    Victory,
    Defeat,
}

pub struct GameOutcomeSounds {
    /// Clip for `GameOutcomeSound::Victory`. Add a field here per new variant.
    victory: Option<Sound>,
    /// Clip for `GameOutcomeSound::Defeat`. Add a field here per new variant.
    defeat: Option<Sound>,
}

impl GameOutcomeSounds {
    pub fn new() -> Self {
        Self {
            victory: None,
            defeat: None,
        }
    }
}

impl AudioClips for GameOutcomeSounds {
    type Kind = GameOutcomeSound;

    /// Exhaustive match: adding a `GameOutcomeSound` variant without
    /// adding it here is a compile error.
    async fn load_clip_for(&mut self, clip_kind: &GameOutcomeSound) {
        match clip_kind {
            GameOutcomeSound::Victory => {
                if self.victory.is_none() {
                    println!("{LOG_PREFIX} loading {clip_kind:?}...");

                    self.victory = Some(
                        audio::load_sound_from_bytes(include_bytes!(
                            "../../../../assets/audio/outcome/victory.wav"
                        ))
                        .await
                        .unwrap_or_else(|_| panic!("{LOG_PREFIX} failed to load {clip_kind:?}")),
                    );

                    println!("{LOG_PREFIX} {clip_kind:?} load complete");
                }
            }
            GameOutcomeSound::Defeat => {
                if self.defeat.is_none() {
                    println!("{LOG_PREFIX} loading {clip_kind:?}...");

                    self.defeat = Some(
                        audio::load_sound_from_bytes(include_bytes!(
                            "../../../../assets/audio/outcome/defeat.wav"
                        ))
                        .await
                        .unwrap_or_else(|_| panic!("{LOG_PREFIX} failed to load {clip_kind:?}")),
                    );

                    println!("{LOG_PREFIX} {clip_kind:?} load complete");
                }
            }
        }
    }

    fn get_clip_for(&self, clip_kind: &GameOutcomeSound) -> Option<&Sound> {
        match clip_kind {
            GameOutcomeSound::Victory => self.victory.as_ref(),
            GameOutcomeSound::Defeat => self.defeat.as_ref(),
        }
    }
}
