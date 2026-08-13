use macroquad::prelude::*;

use crate::game::{
    asset_repository::{
        traits::Singleton,
        sprite_repository::{traits::{SpriteTextures, SpriteBounds}, SpriteRepository, BannerV1Textures},
    },
    entities::animation::Animation,
    rendering::Drawable,
};

/// Which title banner to draw. Callers pick a `BannerKind`; the mapping to the
/// underlying texture asset stays internal to this module, so swapping art or
/// adding banners never touches call sites.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BannerKind {
    /// The main title on the home/title screen.
    HomeTitle,
    /// The heading on the level-selection page.
    LevelSelectionTitle,
    /// The heading on the ship-selection page.
    ShipSelectionTitle,
}

impl BannerKind {
    /// The texture this banner draws.
    fn texture(&self) -> BannerV1Textures {
        match self {
            BannerKind::HomeTitle => BannerV1Textures::HomePageMain,
            BannerKind::LevelSelectionTitle => BannerV1Textures::LevelSelectionPageMain,
            BannerKind::ShipSelectionTitle => BannerV1Textures::ShipSelectionPageMain,
        }
    }
}

impl SpriteBounds for BannerKind {
    /// Delegates to the backing texture so callers can size a banner by its
    /// art's aspect ratio without naming the texture enum directly.
    fn content_bounds(&self) -> Rect {
        self.texture().content_bounds()
    }
}

/// A static heading image, e.g. a page's title screen banner.
///
/// # Example
///
/// ```no_run
/// use asteroid_rush::game::{
///     rendering::Drawable,
///     ui::components::banner::{Banner, BannerKind},
/// };
/// use macroquad::prelude::*;
///
/// async fn demo() {
///     let title = Banner::new(
///         BannerKind::HomeTitle,
///         Vec2::new(20.0, 50.0),
///         Vec2::new(450.0, 111.0),
///     );
///
///     loop {
///         // A banner is static, so there's no per-frame state to update —
///         // just redraw it each frame at its fixed position.
///         title.draw();
///         next_frame().await;
///     }
/// }
/// ```
pub struct Banner {
    animation: Animation,
    position: Vec2,
}

impl Banner {
    pub fn new(kind: BannerKind, position: Vec2, size: Vec2) -> Self {
        let banner_sprites = &SpriteRepository::get_instance().banner_v1_sprite;
        Self {
            animation: Animation::new(
                banner_sprites.get_textures_for(&kind.texture()),
                size,
                1.0,
                None,
            ),
            position,
        }
    }
}

impl Drawable for Banner {
    fn draw(&self) {
        draw_texture_ex(
            self.animation.current_frame(),
            self.position.x,
            self.position.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(*self.animation.frame_scale()),
                ..Default::default()
            },
        );
    }
}
