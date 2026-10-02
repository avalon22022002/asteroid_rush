use crate::game::entities::bullet::bullet_kind::BulletKind;

#[derive(Debug, Clone)]
pub struct BulletStats {
    damage: u32,
    speed: f32,
}

impl BulletStats {
    pub fn stats_for(bullet_kind: BulletKind) -> BulletStats {
        match bullet_kind {
            BulletKind::BlueLaser => BulletStats {
                damage: 30,
                speed: 180.0,
            },
            BulletKind::RedLaser => BulletStats {
                damage: 40,
                speed: 200.0,
            },
        }
    }

    pub fn damage(&self) -> u32 {
        self.damage
    }

    pub fn speed(&self) -> f32 {
        self.speed
    }
}
