pub mod bullet_kind;
pub mod bullet_stats;
pub mod bullet_textures;
pub mod utils;

use macroquad::prelude::*;

use crate::game::{
    animation::Animation,
    entities::bullet::{bullet_kind::BulletKind, bullet_stats::BulletStats},
    traits::rendering::Drawable,
};

pub struct Bullet {
    bounds: Rect,
    kind: BulletKind,
    stats: BulletStats,
    animation: Animation
}

impl Bullet {
    pub fn new(pos: Vec2, kind: BulletKind) -> Self{
        let animation = kind.animation();
        let size = *animation.frame_scale();
        Self {
            bounds: Rect { x: pos.x, y: pos.y, w: size.x, h: size.y },
            stats: BulletStats::stats_for(kind),
            animation,
            kind,
        }
    }
}

impl Drawable for Bullet {
    fn draw(&self) {
        draw_texture_ex(
            self.animation.current_frame(),
            self.bounds.x,
            self.bounds.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(self.bounds.size()),
                ..Default::default()
            },
        );
    }
}
