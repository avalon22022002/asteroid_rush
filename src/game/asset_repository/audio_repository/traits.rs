use macroquad::audio::Sound;
use strum::IntoEnumIterator;

/// Common lazy-loading behavior shared by `audio_repository`'s clip-group
/// types (`ButtonClickSounds`, ...): each implementor names its clips with
/// an enum (`Kind`) and always reads them back via `get_clip_for`. Two ways
/// to load first: `load_clip_for` + `get_clip_for`, one clip at a time, on
/// demand; or `load_all_clips` + `get_clip_for`, everything up front at
/// game init so nothing stalls decoding mid-gameplay.
#[allow(async_fn_in_trait)]
pub trait AudioClips {
    /// Enumerates this type's named clips (e.g. `ButtonClickSound`).
    type Kind: IntoEnumIterator;

    /// Loads the clip for `clip_kind` on first use. Implementers should
    /// make this idempotent, since a redundant call shouldn't re-decode a
    /// clip already loaded. Use `get_clip_for` to read it back.
    async fn load_clip_for(&mut self, clip_kind: &Self::Kind);

    /// Eagerly loads every `Kind` variant. Call once at game init, same as
    /// `Preloadable::load_all`.
    async fn load_all_clips(&mut self) {
        for clip_kind in Self::Kind::iter() {
            self.load_clip_for(&clip_kind).await;
        }
    }

    /// Returns the clip for `clip_kind`, or `None` if it hasn't been loaded
    /// yet. `load_clip_for`/`load_all_clips` must be called (and awaited)
    /// first.
    fn get_clip_for(&self, clip_kind: &Self::Kind) -> Option<&Sound>;
}
