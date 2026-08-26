use crate::game::entities::bullet::bullet_kind::BulletKind;

pub struct BulletStats {
    damage: f32,
}

impl BulletStats {
    pub fn stats_for(bullet_kind: BulletKind) -> BulletStats {
        match bullet_kind {
            BulletKind::BlueLaser => BulletStats{damage: 30.0},
            BulletKind::RedLaser => BulletStats{damage: 40.0}
        }
    }
}
