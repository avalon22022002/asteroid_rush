//! Central asset repository, following the repository pattern: every game
//! asset (audio, textures, ...) is loaded and cached in one place here,
//! rather than each entity/component that uses it decoding its own copy.
//! Callers ask a submodule for an asset by name instead of touching
//! `include_bytes!`/decoding themselves.

pub mod audio_repository;
pub mod sprite_repository;
pub mod traits;
