use macroquad::math::Rect;

use crate::game::{
    animation::Animation,
    asset_repository::{
        sprite_repository::{traits::{SpriteTextures, SpriteBounds}, SpriteRepository, ShipV1Textures},
        traits::Singleton,
    },
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShipKind {
    // Allrounder: 1 Gun, Medium Health, Medium Speed
    Vanguard,
    // Defender: 3 Guns, High Health, Slow
    Sentinel,
    // Attacker: 3 Guns, Low Health, Fast
    Viper,
}

impl ShipKind {
    /// Display name shown in the ship-select UI.
    pub fn display_name(&self) -> &'static str {
        match self {
            ShipKind::Vanguard => "Vanguard",
            ShipKind::Sentinel => "Sentinel",
            ShipKind::Viper => "Viper",
        }
    }

    /// One-line role blurb shown under `display_name` in the ship-select UI.
    pub fn role(&self) -> &'static str {
        match self {
            ShipKind::Vanguard => "Allrounder",
            ShipKind::Sentinel => "Defender",
            ShipKind::Viper => "Attacker",
        }
    }

    /// Creates the alive and dead animations for this kind, sized to fit as
    /// closely as possible inside `approx_bounds` without stretching,
    /// squashing, or cropping the sprite. Also returns that fitted box,
    /// which becomes the ship's starting bounds.
    pub fn get_alive_and_dead_animations_for(
        self,
        approx_bounds: Rect,
    ) -> (Animation, Animation, Rect) {
        let ship_sprites = &SpriteRepository::get_instance().ship_v1_sprite;

        match self {
            ShipKind::Sentinel => {
                let sprite = ShipV1Textures::SentinelAlive;
                let fitted_bounds = sprite.fit_centered_in(approx_bounds);
                let crop = Some(sprite.content_bounds());

                let alive = Animation::new(
                    ship_sprites.get_textures_for(&sprite),
                    fitted_bounds.size(),
                    12.0,
                    crop,
                );

                // No dedicated death sprite set yet — freeze on the last
                // alive frame as a placeholder until one is added.
                let dead = Animation::new(
                    ship_sprites.get_textures_for(&sprite),
                    fitted_bounds.size(),
                    1.0,
                    crop,
                );

                (alive, dead, fitted_bounds)
            }

            ShipKind::Vanguard => todo!("vanguard animation frames not added yet"),
            ShipKind::Viper => todo!("viper animation frames not added yet"),
        }
    }
}
