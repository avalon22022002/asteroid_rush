use std::collections::HashMap;
use std::sync::OnceLock;

use macroquad::audio::{self, Sound};

/// Identifies a sound effect the game can play. Add a variant here and an
/// entry in `sound_sources` to register a new sound.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AudioName {
    ButtonClick,
    ButtonHover,
}

/// Maps each `AudioName` to its embedded asset bytes. `include_bytes!` bakes
/// the files in at compile time, so a missing/renamed asset fails the build
/// instead of surfacing as a runtime error — and it helps in wasm/web
/// builds, which can't do plain file-system reads.
fn sound_sources() -> [(AudioName, &'static [u8]); 2] {
    [
        (
            AudioName::ButtonClick,
            include_bytes!("../../assets/audio/button/button-click.wav"),
        ),
        (
            AudioName::ButtonHover,
            include_bytes!("../../assets/audio/button/button-hover.wav"),
        ),
    ]
}

/// Decoded sounds, keyed by name. Populated once by `load_sounds` and read
/// by `play` from anywhere in the game.
static SOUNDS: OnceLock<HashMap<AudioName, Sound>> = OnceLock::new();

/// Decodes every registered sound effect. Must be awaited once at game init
/// (e.g. the top of `main`, before the game loop starts) so `play` has
/// something to play.
pub async fn load_sounds() {
    println!("[audio] loading...");

    let mut map = HashMap::new();
    for (name, bytes) in sound_sources() {
        println!("[audio] loading {name:?}...");

        let sound = audio::load_sound_from_bytes(bytes)
            .await
            .unwrap_or_else(|e| panic!("failed to decode sound {name:?}: {e}"));

        map.insert(name, sound);
    }
    let _ = SOUNDS.set(map);

    println!("[audio] load complete");
}

/// Plays the given sound effect once. No-ops instead of panicking if
/// `load_sounds` hasn't finished yet, so a slow load never crashes an early
/// call site.
pub fn play(name: AudioName) {
    if let Some(sound) = SOUNDS.get().and_then(|map| map.get(&name)) {
        audio::play_sound_once(sound);
    }
}
