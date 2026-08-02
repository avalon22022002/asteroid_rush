//! Central asset repository, following the repository pattern: every game
//! asset (audio, textures, ...) is loaded and cached in one place here,
//! rather than each entity/component that uses it decoding its own copy.
//! Callers ask a submodule for an asset by name instead of touching
//! `include_bytes!`/decoding themselves.

pub mod audio_repository;
pub mod textures;

/// A single category of asset (audio, textures, ...) as a repository. Each
/// implementor defines its own fixed source table — one entry per
/// `AssetIdentifier` variant, paired with its bytes via `include_bytes!`.
/// That only checks the file exists and embeds its raw bytes at compile
/// time (a missing/renamed path fails the build); decoding those bytes into
/// a usable asset still happens at runtime, via `load_all`/`get_asset`.
#[allow(async_fn_in_trait)]
pub trait AssetRepository {
    /// The decoded, ready-to-use asset type this repository stores
    /// (e.g. `Texture2D`, `Sound`).
    type Asset;
    /// An enum of every asset this repository recognizes (e.g. `AudioName`),
    /// so call sites look assets up by variant instead of a raw string.
    type AssetIdentifier;

    /// Eagerly decodes every asset in this repository's source table. Call
    /// once at game init (e.g. the top of `main`) so later `get_asset`
    /// calls don't stall the frame decoding on demand. Implementers should
    /// make this idempotent, since a redundant call shouldn't re-decode
    /// anything already loaded.
    async fn load_all(&mut self);

    /// Looks up a decoded asset by identifier. `load_all` must be called
    /// (and awaited) before `get_asset`. Calling `get_asset` before
    /// initialization is a contract violation; implementations may panic.
    fn get_asset(&self, identifier: Self::AssetIdentifier) -> &Self::Asset;
}
