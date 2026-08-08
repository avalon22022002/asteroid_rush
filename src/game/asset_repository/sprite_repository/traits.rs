use macroquad::texture::Texture2D;
use strum::IntoEnumIterator;

/// Common lazy-loading behavior shared by `sprite_repository`'s texture-group
/// types (`ButtonV1`, `ShipV1`, ...): each implementor names its texture
/// groups with an enum (`Kind`) and loads them lazily, one group at a time,
/// via `load_textures_for`.
#[allow(async_fn_in_trait)]
pub trait SpriteTextures {
    /// Enumerates this type's named texture groups (e.g. `ButtonV1Textures`).
    type Kind: IntoEnumIterator;

    /// Loads the frames for `texture_kind` on first use. Implementers should
    /// make this idempotent, since a redundant call shouldn't re-decode
    /// frames already loaded. Use `get_textures_for` to read them back.
    async fn load_textures_for(&mut self, texture_kind: &Self::Kind);

    /// Eagerly loads every `Kind` variant. Call once at game init, same as
    /// `Preloadable::load_all`.
    async fn load_all_textures(&mut self) {
        for texture_kind in Self::Kind::iter() {
            self.load_textures_for(&texture_kind).await;
        }
    }

    /// Returns the frames for `texture_kind`. `load_textures_for`/
    /// `load_all_textures` must be called (and awaited) first, else it may
    /// return empty.
    fn get_textures_for(&self, texture_kind: &Self::Kind) -> &Vec<Texture2D>;
}

/// Every sprite type implements this. Just `SpriteTextures` for now; more
/// capability traits will join as supertraits later.
pub trait Sprite: SpriteTextures {}