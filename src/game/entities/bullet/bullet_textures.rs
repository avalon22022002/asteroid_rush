use macroquad::{math::Rect, texture::Texture2D};

use crate::game::entities::bullet::{bullet_kind::BulletKind, utils::{self, laser::{LaserTextureParams, PrimaryColor::{Blue, Red}}}};

pub enum BulletV1Textures {
    Basic,
}

impl BulletV1Textures {
    pub fn textures_for(bullet_kind: BulletKind) -> Vec<Texture2D> {
        match bullet_kind {
            BulletKind::BlueLaser => {
                utils::laser::build_laser_textures(LaserTextureParams{primary_color: Blue}).textures
            },
            BulletKind::RedLaser => {
                utils::laser::build_laser_textures(LaserTextureParams{primary_color: Red}).textures
            }
        }
    }

    pub fn bounds_for(bullet_kind: BulletKind) -> Rect {
        match bullet_kind {
            BulletKind::BlueLaser => {
                utils::laser::build_laser_textures(LaserTextureParams{primary_color: Blue}).bounds
            },
            BulletKind::RedLaser => {
                utils::laser::build_laser_textures(LaserTextureParams{primary_color: Red}).bounds
            }
        }
    }
}
