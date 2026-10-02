pub mod traits;
pub mod button_click;
pub mod button_hover;
pub mod gun_fire;
pub mod asteroid_destruction;
pub mod asteroid_hit;
pub mod ship_destruction;
pub mod ship_damage;

use std::sync::OnceLock;

use futures::executor;

use button_click::ButtonClickSounds;
use button_hover::ButtonHoverSounds;
use gun_fire::GunFireSounds;
use asteroid_destruction::AsteroidDestructionSounds;
use asteroid_hit::AsteroidHitSounds;
use ship_destruction::ShipDestructionSounds;
use ship_damage::ShipDamageSounds;
use crate::game::asset_repository::{
    audio_repository::traits::AudioClips,
    traits::{Preloadable, Singleton},
};

/// # Example
///
/// ```no_run
/// use asteroid_rush::game::asset_repository::{
///     traits::Singleton,
///     audio_repository::{AudioRepository, traits::AudioClips, button_click::ButtonClickSound},
/// };
///
/// // get_instance builds (and loads) the repository on first call, so it's
/// // already fully loaded here — no separate load_all step needed.
/// let repo = AudioRepository::get_instance();
/// let clip = repo.button_click_sounds.get_clip_for(&ButtonClickSound::Basic);
/// ```
pub struct AudioRepository {
    pub button_click_sounds: ButtonClickSounds,
    pub button_hover_sounds: ButtonHoverSounds,
    pub gun_fire_sounds: GunFireSounds,
    pub asteroid_destruction_sounds: AsteroidDestructionSounds,
    pub asteroid_hit_sounds: AsteroidHitSounds,
    pub ship_destruction_sounds: ShipDestructionSounds,
    pub ship_damage_sounds: ShipDamageSounds,
}

impl AudioRepository {
    pub fn new() -> Self {
        Self {
            button_click_sounds: ButtonClickSounds::new(),
            button_hover_sounds: ButtonHoverSounds::new(),
            gun_fire_sounds: GunFireSounds::new(),
            asteroid_destruction_sounds: AsteroidDestructionSounds::new(),
            asteroid_hit_sounds: AsteroidHitSounds::new(),
            ship_destruction_sounds: ShipDestructionSounds::new(),
            ship_damage_sounds: ShipDamageSounds::new(),
        }
    }
}

static INSTANCE: OnceLock<AudioRepository> = OnceLock::new();

impl Singleton for AudioRepository {
    fn storage() -> &'static OnceLock<Self> {
        &INSTANCE
    }

    /// Builds a fully loaded `AudioRepository`. `block_on`'s `load_all` to
    /// stay sync, same tradeoff (and reasoning) as `SpriteRepository::new`.
    fn new() -> Self {
        let mut repo = AudioRepository::new();
        executor::block_on(repo.load_all());
        repo
    }
}

impl Preloadable for AudioRepository {
    /// Loads every clip group concurrently (`futures::join!`) rather than
    /// one after another, so the total wait is the slowest single load, not
    /// their sum.
    async fn load_all(&mut self) {
        futures::join!(
            self.button_click_sounds.load_all_clips(),
            self.button_hover_sounds.load_all_clips(),
            self.gun_fire_sounds.load_all_clips(),
            self.asteroid_destruction_sounds.load_all_clips(),
            self.asteroid_hit_sounds.load_all_clips(),
            self.ship_destruction_sounds.load_all_clips(),
            self.ship_damage_sounds.load_all_clips(),
        );
    }
}
