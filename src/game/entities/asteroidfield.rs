use crate::game::{
    asset_repository::{
        audio_repository::{
            AudioRepository, asteroid_destruction::AsteroidDestructionSound,
            asteroid_hit::AsteroidHitSound, traits::AudioClips,
        },
        traits::Singleton,
    },
    entities::asteroidfield::asteroid::{Asteroid, asteroid_kind::AsteroidKind},
    traits::{
        damage::{DamageResult, Damageable},
        object::HasBoundingCircle,
        rendering::{Drawable, StateUpdatable},
    },
    utils::MinMax,
};
use macroquad::{audio, math::Vec2, window::screen_width};

pub mod asteroid;

const LOG_PREFIX: &str = "[asteroid_field]";

#[derive(Debug, Clone)]
pub struct AsteroidField {
    asteroids: Vec<Asteroid>,
}

impl AsteroidField {
    /// Generates an asteroid field of `asteroid_count` asteroids, with each
    /// field of every asteroid drawn uniformly from the range between the
    /// matching field in `limits.0` and `limits.1` (order doesn't matter —
    /// each field is normalized independently). Defaults to
    /// [`Asteroid::default_bounds`] spanning the full screen width when
    /// `limits` is `None`.
    pub fn new(
        asteroid_count: usize,
        asteroid_kind: AsteroidKind,
        scale_limits: MinMax<f32>,
    ) -> AsteroidField {
        let asteroids = (0..asteroid_count)
            .map(|_| {
                let random_x_coordinate = macroquad::rand::gen_range(0.0, screen_width());
                let random_y_cordinate = macroquad::rand::gen_range(0.0, 10.0);
                let random_scale = macroquad::rand::gen_range(scale_limits.min, scale_limits.max);

                // New asteroid at near screen top  with 0 initial rotation, random scale and random x,y coordinate
                Asteroid::new(
                    Vec2::new(random_x_coordinate, random_y_cordinate),
                    0.0,
                    asteroid_kind,
                    random_scale,
                )
            })
            .collect();

        AsteroidField { asteroids }
    }

    /// Resolves collisions between active asteroids and `other`.
    ///
    /// - Brute-force checks every asteroid in the field; the small asteroid count
    ///   makes this approach sufficient for the game.
    /// - Respawns each asteroid whose collision circle overlaps `other`.
    /// - Returns the total damage dealt by all collisions, or `0` if none occur.
    /// - Accepts any type implementing `HasBoundingCircle` (e.g. ship, bullet).
    /// - The caller is responsible for applying the returned damage.
    pub fn resolve_collision<T: HasBoundingCircle>(&mut self, other: &T) -> u32 {
        let mut damage = 0;
        for asteroid in self.asteroids.iter_mut() {
            if asteroid.is_active() && asteroid.bounding_circle_overlaps(other) {
                let hit_damage = asteroid.damage_on_collision();
                println!("{LOG_PREFIX} resolved a collision, dealt {hit_damage} damage");
                damage += hit_damage;
                asteroid.respawn();
            }
        }
        damage
    }

    /// Resolves a collision between `bullet` and (at most) the first active
    /// asteroid it overlaps — a bullet can't pass through an asteroid, so it
    /// won't hit multiple in one frame. Applies `bullet_damage` to that
    /// asteroid's health, adding its point value to `score` if this kills it.
    ///
    /// Returns whether the bullet hit something, so the caller can drop it.
    pub fn resolve_bullet_collision<T: HasBoundingCircle>(
        &mut self,
        bullet: &T,
        bullet_damage: u32,
        score: &mut u32,
    ) -> bool {
        for asteroid in self.asteroids.iter_mut() {
            if asteroid.is_active() && asteroid.bounding_circle_overlaps(bullet) {
                if asteroid.take_damage(bullet_damage) == DamageResult::Destroyed {
                    let points = asteroid.points_on_destruction();
                    *score += points;
                    println!(
                        "{LOG_PREFIX} resolved a bullet collision, destroyed asteroid worth {points} points"
                    );

                    // play the asteroid destruction sound
                    audio::play_sound_once(
                        AudioRepository::get_instance()
                            .asteroid_destruction_sounds
                            .get_clip_for(&AsteroidDestructionSound::Retro)
                            .unwrap_or_else(|| {
                                panic!("{LOG_PREFIX} asteroid destruction sound not loaded")
                            }),
                    );
                } else {
                    println!(
                        "{LOG_PREFIX} resolved a bullet collision, dealt {bullet_damage} damage"
                    );

                    // play the asteroid hit sound
                    audio::play_sound_once(
                        AudioRepository::get_instance()
                            .asteroid_hit_sounds
                            .get_clip_for(&AsteroidHitSound::Tick)
                            .unwrap_or_else(|| {
                                panic!("{LOG_PREFIX} asteroid hit sound not loaded")
                            }),
                    );
                }
                return true;
            }
        }
        false
    }
}

impl Default for AsteroidField {
    fn default() -> Self {
        AsteroidField::new(
            20,
            AsteroidKind::MoltenDarkAsteroid,
            MinMax { min: 0.3, max: 2.0 },
        )
    }
}

impl Drawable for AsteroidField {
    fn draw(&self) {
        for asteroid in &self.asteroids {
            asteroid.draw();
        }
    }
}

impl StateUpdatable<()> for AsteroidField {
    fn update_state(&mut self, data: ()) {
        for asteroid in self.asteroids.iter_mut() {
            asteroid.update_state(data);
        }
    }
}
