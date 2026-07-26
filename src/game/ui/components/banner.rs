use macroquad::prelude::*;

use crate::game::rendering::Drawable;

const LOG_PREFIX: &str = "[banner]";

/// Identifies which of the game's banner images to show by default when
/// `Banner::new` is given `None` for `texture`. Add a variant here (and a
/// case in `BannerKind::default_texture`) to register a new banner.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BannerKind {
    /// The home page's main title banner ("AstroRush: Space Shooter Classic").
    HomePageMain,
}

impl BannerKind {
    /// Decodes this kind's default banner image. `include_bytes!` bakes the
    /// file in at compile time, so a missing/renamed asset fails the build
    /// instead of surfacing as a runtime error.
    fn default_texture(self) -> Texture2D {
        match self {
            BannerKind::HomePageMain => {
                println!(
                    "{LOG_PREFIX} loading {self:?} (assets/ui/text/home_page_title.png)..."
                );
                Texture2D::from_file_with_format(
                    include_bytes!("../../../../assets/ui/text/home_page_title.png"),
                    None,
                )
            }
        }
    }
}

/// A static heading image, e.g. a page's title screen banner.
pub struct Banner {
    texture: Texture2D,
    position: Vec2,
    size: Vec2,
}

impl Banner {
    /// `kind` picks the default banner art (see `BannerKind`) drawn at
    /// `position`, stretched to `size`. Pass `texture` as `Some(..)` to
    /// override `kind`'s default with custom art instead.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let title = Banner::new(
    ///     BannerKind::HomePageMain,
    ///     Vec2::new(20.0, 50.0),
    ///     Vec2::new(450.0, 111.0),
    ///     None, // use HomePageMain's default art
    /// );
    /// ```
    pub fn new(kind: BannerKind, position: Vec2, size: Vec2, texture: Option<Texture2D>) -> Self {
        Self {
            texture: texture.unwrap_or_else(|| kind.default_texture()),
            position,
            size,
        }
    }
}

impl Drawable for Banner {
    fn draw(&self) {
        draw_texture_ex(
            &self.texture,
            self.position.x,
            self.position.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(self.size),
                ..Default::default()
            },
        );
    }
}
