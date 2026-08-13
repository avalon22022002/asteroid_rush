use macroquad::prelude::*;

use crate::game::{
    BASE_HEIGHT, BASE_WIDTH, 
    asset_repository::{
        sprite_repository::{traits::{SpriteTextures, SpriteBounds}, SpriteRepository, ShipV1Textures},
        traits::Singleton
    }, 
    entities::animation::Animation,
    rendering::{Drawable, StateUpdatable},
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
    pub fn display_name(self) -> &'static str {
        match self {
            ShipKind::Vanguard => "Vanguard",
            ShipKind::Sentinel => "Sentinel",
            ShipKind::Viper => "Viper",
        }
    }

    /// One-line role blurb shown under `display_name` in the ship-select UI.
    pub fn role(self) -> &'static str {
        match self {
            ShipKind::Vanguard => "Allrounder",
            ShipKind::Sentinel => "Defender",
            ShipKind::Viper => "Attacker",
        }
    }

    /// Base `ShipStats` for a freshly spawned ship of this kind, matching
    /// the class blurbs above (gun count, relative health, relative speed).
    /// Also drives the ship-select preview, so it's public.
    pub fn stats(self) -> ShipStats {
        match self {
            ShipKind::Vanguard => ShipStats {
                max_health: 100.0,
                cur_health: 100.0,
                speed: 200.0,
                gun_count: 1,
                fire_damage: 10.0,
            },
            ShipKind::Sentinel => ShipStats {
                max_health: 150.0,
                cur_health: 150.0,
                speed: 120.0,
                gun_count: 2,
                fire_damage: 8.0,
            },
            ShipKind::Viper => ShipStats {
                max_health: 60.0,
                cur_health: 60.0,
                speed: 280.0,
                gun_count: 3,
                fire_damage: 14.0,
            },
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ShipStats {
    max_health: f32,
    cur_health: f32,
    speed: f32,
    gun_count: u8,
    fire_damage: f32,
}

impl ShipStats {
    pub fn max_health(&self) -> f32 {
        self.max_health
    }
    pub fn speed(&self) -> f32 {
        self.speed
    }
    pub fn gun_count(&self) -> u8 {
        self.gun_count
    }
    pub fn fire_damage(&self) -> f32 {
        self.fire_damage
    }
}
pub struct Ship {
    pos: Vec2,
    kind: ShipKind,
    ship_stats: ShipStats,
    description: String,
    alive_animation: Animation,
    dead_animation: Animation,
    is_alive: bool,
}

impl Ship {
    pub fn get_alive_and_dead_animations_for( kind:ShipKind ) -> (Animation, Animation) {
        let ship_sprites = &SpriteRepository::get_instance().ship_v1_sprite;
        match kind {
            ShipKind::Sentinel => {
                // Crop the transparent padding so the drawn ship fills its
                // scale box instead of floating small inside the frame.
                let crop = Some(ShipV1Textures::SentinelAlive.content_bounds());
                let alive = Animation::new(ship_sprites.get_textures_for(&ShipV1Textures::SentinelAlive), Vec2::new(236.0, 300.0), 12.0, crop);
                // No dedicated death sprite set yet — freeze on the last
                // alive frame as a placeholder until one's added.
                let dead = Animation::new(
                    ship_sprites.get_textures_for(&ShipV1Textures::SentinelAlive),
                    Vec2::new(236.0, 300.0),
                    1.0,
                    crop,
                );
                return (alive, dead)
            }
            ShipKind::Vanguard => todo!("vanguard animation frames not added yet"),
            ShipKind::Viper => todo!("viper animation frames not added yet"),
        }
    }

    pub fn new(pos: Vec2, kind: ShipKind, description: String) -> Self {
        let (alive_animation, dead_animation) = Self::get_alive_and_dead_animations_for(kind);
        Self {
            pos,
            kind,
            ship_stats: kind.stats(),
            description,
            alive_animation,
            dead_animation,
            is_alive: true,
        }
    }

    /// Moves the ship by this frame's arrow-key input at `ship_stats.speed`
    /// units/second, clamped so it can't drift outside the game's logical
    /// `BASE_WIDTH`x`BASE_HEIGHT` bounds.
    fn apply_movement(&mut self, dt: f32) {
        let dir = movement_input();
        self.pos = (self.pos + dir * self.ship_stats.speed * dt).clamp(
            Vec2::ZERO,
            Vec2::new(BASE_WIDTH - self.ship_size().x, BASE_HEIGHT - self.ship_size().y),
        );
    }

    /// The animation that reflects the ship's current `is_alive` state.
    fn current_animation(&self) -> &Animation {
        if self.is_alive {
            &self.alive_animation
        } else {
            &self.dead_animation
        }
    }

    fn ship_size(&self) -> &Vec2 {
        if self.is_alive {
            &self.alive_animation.frame_scale()
        } else {
            &self.dead_animation.frame_scale()
        }
    }

    fn current_animation_mut(&mut self) -> &mut Animation {
        if self.is_alive {
            &mut self.alive_animation
        } else {
            &mut self.dead_animation
        }
    }
}

/// Reads the arrow keys and returns a direction vector for this frame's
/// movement input, normalized so diagonal movement isn't faster than
/// moving along a single axis. Zero if no arrow key is held.
fn movement_input() -> Vec2 {
    let mut dir = Vec2::ZERO;
    if is_key_down(KeyCode::Left) {
        dir.x -= 1.0;
    }
    if is_key_down(KeyCode::Right) {
        dir.x += 1.0;
    }
    if is_key_down(KeyCode::Up) {
        dir.y -= 1.0;
    }
    if is_key_down(KeyCode::Down) {
        dir.y += 1.0;
    }
    if dir != Vec2::ZERO {
        dir = dir.normalize();
    }
    dir
}

impl Drawable for Ship {
    fn draw(&self) {
        draw_texture_ex(
            self.current_animation().current_frame(),
            self.pos.x,
            self.pos.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(*self.ship_size()),
                source: self.current_animation().frame_crop(),
                ..Default::default()
            },
        );
    }
}

impl StateUpdatable<()> for Ship {
    /// Only responds to movement input while alive — a dead ship shouldn't
    /// steer, just play out its death animation in place.
    fn update_state(&mut self, _data: ()) {
        let dt = get_frame_time();
        if self.is_alive {
            self.apply_movement(dt);
        }
        self.current_animation_mut().advance(dt);
    }
}
