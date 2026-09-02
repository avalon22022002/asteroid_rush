use macroquad::math::Rect;

use crate::game::{
    animation::Animation,
    asset_repository::{
        sprite_repository::{traits::{SpriteTextures, SpriteBounds}, SpriteRepository, ShipV1Textures},
        traits::Singleton,
    },
    entities::bullet::bullet_kind::BulletKind,
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

    /// The bullet kind this ship's guns fire.
    pub fn bullet_kind(&self) -> BulletKind {
        match self {
            ShipKind::Vanguard => BulletKind::BlueLaser,
            ShipKind::Sentinel => BulletKind::BlueLaser,
            ShipKind::Viper => BulletKind::RedLaser,
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
                let alive_sprite = ShipV1Textures::SentinelAlive;
                let fitted_bounds = alive_sprite.fit_centered_in(approx_bounds);

                let alive = Animation::new(
                    ship_sprites.get_textures_for(&alive_sprite),
                    fitted_bounds.size(),
                    12.0,
                    Some(alive_sprite.content_bounds()),
                );

                let dead_sprite = ShipV1Textures::SentinelDead;
                let dead = Animation::new(
                    ship_sprites.get_textures_for(&dead_sprite),
                    fitted_bounds.size(),
                    12.0,
                    Some(dead_sprite.content_bounds()),
                ).play_once();

                (alive, dead, fitted_bounds)
            }

            ShipKind::Vanguard => todo!("vanguard animation frames not added yet"),
            ShipKind::Viper => todo!("viper animation frames not added yet"),
        }
    }
}
