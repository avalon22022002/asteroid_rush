use crate::game::{animation::Animation, entities::bullet::bullet_textures::BulletV1Textures};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BulletKind {
    BlueLaser,
    RedLaser,
}

impl BulletKind {
    pub fn animation(self) -> Animation {
        let scale = BulletV1Textures::bounds_for(self).size();
        Animation::new(&BulletV1Textures::textures_for(self), scale, 1.0, None)
    }
}
