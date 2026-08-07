pub mod audio;

use std::collections::HashMap;
use std::sync::OnceLock;

use futures::executor;
use macroquad::logging;
use strum::IntoEnumIterator;

use crate::game::asset_repository::{
    traits::AssetRepository,
    audio_repository::audio::{Audio, AudioIdentifier},
};

const LOG_PREFIX: &str = "AudioRepository: ";

#[derive(Debug)]
pub struct AudioRepository {
    audios: OnceLock<HashMap<AudioIdentifier, Audio>>,
}

/// The shared singleton returned by `get_instance`.
static INSTANCE: OnceLock<AudioRepository> = OnceLock::new();

impl AudioRepository {
    /// Builds a fully loaded `AudioRepository`. `block_on`'s the async
    /// `load_all` to stay sync — fine here since this repo is meant to be
    /// initialized once, up front, before the game loop starts: a slightly
    /// longer startup trades off for a lag-free game loop afterward.
    fn new() -> AudioRepository {
        let mut repo = AudioRepository {
            audios: OnceLock::new(),
        };
        executor::block_on(repo.load_all());
        repo
    }

    /// Returns the shared singleton, building and loading it on first call.
    /// Prefer calling this once at game init so that cost lands there
    /// instead of mid-game — but once set, calling this again (as often as
    /// you like, from anywhere) is just a cheap lookup.
    pub fn get_instance() -> &'static AudioRepository {
        if let Some(instance) = INSTANCE.get() {
            return instance;
        }

        INSTANCE.set(AudioRepository::new()).unwrap_or_else(|_| panic!("{LOG_PREFIX} get_instance failed to set the shared instance"));

        INSTANCE
            .get()
            .unwrap_or_else(|| panic!("{LOG_PREFIX} get_instance failed to initialize"))
    }
}

impl AssetRepository for AudioRepository {
    type Asset = Audio;
    type AssetIdentifier = AudioIdentifier;

    async fn load_all(&mut self) {
        if self.audios.get().is_some() {
            let msg = format!("{LOG_PREFIX} Already Iniliazed skipping");
            logging::info!("{msg}");
        }

        logging::info!("{LOG_PREFIX} initializing..");
        let mut audio_map: HashMap<AudioIdentifier, Audio> = HashMap::new();
        for identifier in AudioIdentifier::iter() {
            let sound = Audio::decode_sound(identifier).await.unwrap_or_else(|_| panic!("{LOG_PREFIX} Failed to Load Audio: {identifier:?}"));

            audio_map.insert(identifier, Audio::new(identifier, sound));
        }

        self.audios
            .set(audio_map)
            .unwrap_or_else(|_| panic!("{LOG_PREFIX} Failed to initialize"));

        logging::info!("{LOG_PREFIX} initialized.");
    }

    fn get_asset(&self, identifier: Self::AssetIdentifier) -> &Self::Asset {
        let asset = self
            .audios
            .get()
            .unwrap_or_else(|| panic!("{LOG_PREFIX} get_asset called before load_all"))
            .get(&identifier)
            .unwrap_or_else(|| panic!("{LOG_PREFIX} No audio registered for {identifier:?}"));

        asset
    }
}
