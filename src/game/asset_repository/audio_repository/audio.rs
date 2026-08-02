use macroquad::audio::{self, Sound};
use strum::EnumIter;

/// Identifies a sound effect the game can play. Add a variant here and an
/// entry in `sound_sources` to register a new sound.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum AudioName {
    ButtonClick,
    ButtonHover,
}

#[derive(Debug)]
pub struct Audio {
    identifier: AudioName,
    sound: Sound,
}

impl Audio {
    pub fn new(identifier: AudioName, sound: Sound) -> Audio {
        Audio { identifier, sound }
    }

    pub async fn decode_sound(audio_name: AudioName) -> Result<Sound, macroquad::Error> {
        match audio_name {
            AudioName::ButtonClick => {
                audio::load_sound_from_bytes(include_bytes!(
                    "../../../../assets/audio/ui/click.wav"
                ))
                .await
            }
            AudioName::ButtonHover => {
                audio::load_sound_from_bytes(include_bytes!(
                    "../../../../assets/audio/ui/hover.wav"
                ))
                .await
            }
        }
    }
    /// Plays this sound once.
    pub fn play(&self) {
        audio::play_sound_once(&self.sound);
    }
}
