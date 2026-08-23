use macroquad::{math::{Rect, Vec2}, texture::Texture2D};
use strum::IntoEnumIterator;

use crate::game::utils::{aspect_size_from_fixed_height, aspect_size_from_fixed_width};

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

/// Tight box around a sprite's drawn pixels, excluding transparent padding.
/// Implemented on the `*Textures` kind enums (e.g. `ButtonV1Textures`), since
/// bounds are a property of the individual art, not the texture-group type.
pub trait SpriteBounds {
    /// The tightest rectangle enclosing the sprite's drawn (opaque) pixels, in
    /// source-texture pixel coordinates. Kinds with no transparent padding
    /// return the full frame anchored at the origin.
    fn content_bounds(&self) -> Rect;

    /// The drawn content's width-to-height ratio, for sizing a draw region so
    /// it isn't stretched. Derived from `content_bounds`.
    fn aspect_ratio(&self) -> f32 {
        let b = self.content_bounds();
        b.w / b.h
    }

    /// Draw size at scale 1.0 — multiply by a scale factor to get the final
    /// on-screen size. Defaults to `content_bounds`' raw (source-pixel)
    /// size; override to shrink it if that's too large to draw as-is.
    fn content_size_at_logical_unit_scale(&self) -> Vec2 {
        self.content_bounds().size()
    }

    /// Takes `bounds`, the area available for drawing this sprite, and
    /// returns the `Rect` where the sprite should actually be drawn.
    ///
    /// The returned `Rect` is the largest size that fits completely inside
    /// `bounds` while preserving the sprite's `aspect_ratio`. If `bounds` is
    /// smaller than the sprite, the sprite is scaled down. If `bounds` is
    /// larger, the sprite is scaled up until it can no longer grow without
    /// exceeding `bounds`.
    ///
    /// If the sprite doesn't have the same aspect ratio as `bounds`, the
    /// returned `Rect` is smaller on one axis. It's positioned in the middle
    /// of `bounds`, leaving equal unused space on opposite sides.
    ///
    /// In other words:
    /// - Input:  `bounds` — the approximate area where the sprite should fit.
    /// - Output: `Rect` — the final area the sprite should occupy within `bounds`.
    /// - Result: the sprite is fitted as closely as possible to `bounds` while preserving its aspect ratio,
    ///           so it is never stretched or squashed or cropped.
    fn fit_centered_in(&self, bounds: Rect) -> Rect {
        let aspect_ratio = self.aspect_ratio();
        let by_width = aspect_size_from_fixed_width(bounds.w, aspect_ratio);
        let size = if by_width.y <= bounds.h {
            by_width
        } else {
            aspect_size_from_fixed_height(bounds.h, aspect_ratio)
        };
        let pos = bounds.point() + (bounds.size() - size) / 2.0;
        Rect::new(pos.x, pos.y, size.x, size.y)
    }
}
