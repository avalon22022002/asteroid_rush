use crate::game::{
    entities::asteroidfield::asteroid::asteroid_kind::AsteroidKind,
    utils::{MinMax, biased_random_in_range},
};

#[derive(Debug, Clone)]
pub struct AsteroidStats {
    speed: f32,
    rotation_speed: f32,
    max_health: u32,
    cur_health: u32,
    damage_on_collision: u32,
    spawn_time: u32,
    points_on_destruction: u32,
}

impl AsteroidStats {
    pub fn speed(&self) -> f32 {
        self.speed
    }
    pub fn rotation_speed(&self) -> f32 {
        self.rotation_speed
    }
    pub fn max_health(&self) -> u32 {
        self.max_health
    }
    pub fn cur_health(&self) -> u32 {
        self.cur_health
    }
    pub fn damage_on_collision(&self) -> u32 {
        self.damage_on_collision
    }
    pub fn spawn_time(&self) -> u32 {
        self.spawn_time
    }
    pub fn points_on_destruction(&self) -> u32 {
        self.points_on_destruction
    }

    /// Reduces `cur_health` by `amount`, clamped at 0.
    pub fn apply_damage(&mut self, amount: u32) {
        self.cur_health = self.cur_health.saturating_sub(amount);
    }

    /// Resets `cur_health` back to `max_health` (e.g. when an asteroid respawns).
    pub fn reset_health(&mut self) {
        self.cur_health = self.max_health;
    }

    pub fn range_for(kind: AsteroidKind) -> MinMax<AsteroidStats> {
        match kind {
            AsteroidKind::MoltenDarkAsteroid => MinMax {
                min: AsteroidStats { speed: 50.0, rotation_speed: 1.6, max_health: 20, cur_health: 20, damage_on_collision: 10, spawn_time: 10, points_on_destruction: 10 },
                max: AsteroidStats { speed: 120.0, rotation_speed: 4.2, max_health: 40, cur_health: 40, damage_on_collision: 25, spawn_time: 500, points_on_destruction: 50},
            },
        }
    }

    /// Rolls a fresh, size-appropriate `AsteroidStats` for `kind`, biased by
    /// `scale` (see field comments below for which way each stat leans).
    pub fn random_biased_by_scale_for(kind: AsteroidKind, scale: f32) -> AsteroidStats {
        let stat_range = AsteroidStats::range_for(kind);
        // Bigger asteroids take more hits to destroy: bias grows with size.
        let max_health = biased_random_in_range(MinMax { min: stat_range.min.max_health as f32, max: stat_range.max.max_health as f32 }, scale) as u32;

        AsteroidStats {
            // Bigger asteroids are slower: flip the sign so growing size pulls toward min.
            speed: biased_random_in_range(MinMax { min:stat_range.min.speed, max: stat_range.max.speed }, -scale),
            // Bigger asteroids rotate slower: flip the sign so growing size pulls toward min.
            rotation_speed: biased_random_in_range(MinMax { min: stat_range.min.rotation_speed, max: stat_range.max.rotation_speed }, -scale),
            max_health,
            // Freshly rolled, so current health starts at max.
            cur_health: max_health,
            // Bigger asteroids deal more collision damage: bias grows with size.
            damage_on_collision: biased_random_in_range(MinMax { min: stat_range.min.damage_on_collision as f32, max: stat_range.max.damage_on_collision as f32 }, scale) as u32,
            // Bigger asteroids take longer to spawn: bias grows with scale.
            spawn_time: biased_random_in_range(MinMax { min: stat_range.min.spawn_time as f32, max: stat_range.max.spawn_time as f32 }, scale) as u32,
            // Bigger asteroids are worth more points: bias grows with size.
            points_on_destruction: biased_random_in_range(MinMax { min: stat_range.min.points_on_destruction as f32, max: stat_range.max.points_on_destruction as f32 }, scale) as u32

        }
    }
}
