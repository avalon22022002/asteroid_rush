use macroquad::prelude::*;

use crate::game::{
    asset_repository::{
        traits::Singleton,
        sprite_repository::{traits::SpriteTextures, SpriteRepository, BannerV1Textures},
    },
    entities::animation::Animation,
    rendering::Drawable,
};

/// A static heading image, e.g. a page's title screen banner.
pub struct Banner {
    animation: Animation,
    position: Vec2,
}

impl Banner {
    /// `kind` picks the banner art (see `BannerV1Textures`), drawn at
    /// `position` and stretched to `size`.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let title = Banner::new(
    ///     BannerV1Textures::HomePageMain,
    ///     Vec2::new(20.0, 50.0),
    ///     Vec2::new(450.0, 111.0),
    /// );
    /// ```
    pub fn new(kind: BannerV1Textures, position: Vec2, size: Vec2) -> Self {
        let banner_sprites = &SpriteRepository::get_instance().banner_v1_sprite;
        Self {
            animation: Animation::new(banner_sprites.get_textures_for(&kind), size, 1.0),
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
