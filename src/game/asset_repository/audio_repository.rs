pub mod audio;

use std::fmt::format;
use std::{collections::HashMap};
use std::sync::OnceLock;

use macroquad::logging;
use macroquad::miniquad::log;
use strum::IntoEnumIterator;

use crate::game::{asset_repository};
use crate::game::asset_repository::audio_repository::audio::{Audio,AudioName};

const LOG_PREFIX: &str = "AudioRepository: ";
pub struct AudioRepository {
    audios: OnceLock<HashMap<AudioName, Audio>>,
}

impl asset_repository::AssetRepository for AudioRepository{
    type Asset = Audio;
    type AssetIdentifier = AudioName;

    async fn load_all(&mut self) {
        if self.audios.get().is_some() {
              let msg=format!("{LOG_PREFIX} Already Iniliazed skipping");
              logging::info!("{msg}");
        }

        logging::info!(
            "{LOG_PREFIX} initializing.."
        );
        let mut audio_map:HashMap<AudioName,Audio> = HashMap::new();
        for identifier in AudioName::iter() {
           let sound = Audio::decode_sound(identifier)
            .await
            .expect(&format!("{LOG_PREFIX} Failed to Load Audio: {identifier:?}"));

           audio_map.insert(identifier, Audio::new(identifier, sound));
        }

        self.audios.set(audio_map).expect(&format!("{LOG_PREFIX} Failed to initialize"));

        logging::info!(
            "{LOG_PREFIX} initialized."
        );
    }

    fn get_asset(&self, identifier: Self::AssetIdentifier) -> &Self::Asset {
        let asset=self.audios
        .get()
        .expect(&format!("{LOG_PREFIX} get_asset called before load_all"))
        .get(&identifier)
        .expect(&format!("{LOG_PREFIX} No audio registered for {identifier:?}"));
        
        return asset;
    }
}
