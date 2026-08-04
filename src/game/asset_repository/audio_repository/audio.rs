use macroquad::audio::{self, Sound};
use strum::EnumIter;

/// Identifies a sound effect the game can play. Add a variant here and an
/// entry in `sound_sources` to register a new sound.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum AudioIdentifier {
    ButtonClick,
    ButtonHover,
}

#[derive(Debug)]
pub struct Audio {
    identifier: AudioIdentifier,
    sound: Sound,
}

impl Audio {
    pub fn new(identifier: AudioIdentifier, sound: Sound) -> Audio {
        Audio { identifier, sound }
    }

    pub async fn decode_sound(audio_name: AudioIdentifier) -> Result<Sound, macroquad::Error> {
        match audio_name {
            AudioIdentifier::ButtonClick => {
                audio::load_sound_from_bytes(include_bytes!(
                    "../../../../assets/audio/ui/click.wav"
                ))
                .await
            }
            AudioIdentifier::ButtonHover => {
                audio::load_sound_from_bytes(include_bytes!(
                    "../../../../assets/audio/ui/hover.wav"
                ))
                .await
            }
        }
    }
    pub fn get_sound(&self)->&Sound{
        &self.sound
    }
}
