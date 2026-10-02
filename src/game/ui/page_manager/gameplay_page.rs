use macroquad::{audio, prelude::*};

use crate::game::{
    BASE_HEIGHT, BASE_WIDTH,
    asset_repository::{
        audio_repository::{AudioRepository, game_outcome::GameOutcomeSound, traits::AudioClips},
        sprite_repository::traits::SpriteBounds,
        traits::Singleton,
    },
    entities::{asteroidfield::AsteroidField, ship::Ship},
    game_config::GameConfig,
    traits::{
        damage::Damageable,
        interaction::{Interactive, SelfEventHandler},
        rendering::{Drawable, StateUpdatable},
    },
    ui::components::{
        button::{Button, ButtonEvents, ButtonKind},
        hud::{Hud, HudData},
        overlay_v1::{OverlayV1, OverlayV1Event},
    },
    utils::{MinMax, aspect_size_from_fixed_width},
};

const LOG_PREFIX: &str = "[gameplay_page]";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GamplayPageEvent {
    PauseButtonPressed,
    ResumeButtonPressed,
    ReturnToHomeButtonPressed,
    RetryButtonPressed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GameOutcome {
    Victory,
    Defeat,
}

pub struct GameplayPage {
    /// Whether the game is currently paused.
    paused: bool,
    /// Set once the run ends; freezes gameplay like `paused`, but there's no resuming from it.
    outcome: Option<GameOutcome>,
    /// Built once `outcome` is set, so it can show the final score.
    end_overlay: Option<OverlayV1>,
    pause_button: Button,
    pause_overlay: OverlayV1,
    ship: Ship,
    asteroid_field: AsteroidField,
    /// Points earned by destroying asteroids so far this run.
    score: u32,
    /// Counts down from the level's `duration_secs` to 0; the HUD displays it.
    time_remaining: f32,
    hud: Hud,
}

impl GameplayPage {
    pub fn new(game_config: GameConfig) -> Self {
        // Pause Button
        let pause_button_kind = ButtonKind::Basic;
        let pause_button_size =
            aspect_size_from_fixed_width(90.0, pause_button_kind.aspect_ratio());
        let pause_button_pos = Vec2::new(BASE_WIDTH - pause_button_size.x - 20.0, 15.0);

        // Ship
        let ship_size = Vec2::new(120.0, 150.0);
        let (pos_x, pos_y) = (
            (BASE_WIDTH - ship_size.x) / 2.0,
            (BASE_HEIGHT - ship_size.y) / 2.0,
        );
        let ship_bounds = Rect::new(pos_x, pos_y, ship_size.x, ship_size.y);
        let ship_kind = game_config.ship_kind();

        // Asteroid Field
        let asteroid_count = 20;
        let asteroid_kind = game_config.asteroid_kind();
        let asteroid_scale_limits = MinMax { min: 0.3, max: 2.0 };

        let time_remaining = game_config.level().duration_secs();

        Self {
            paused: false,
            outcome: None,
            end_overlay: None,
            pause_button: Button::new(
                Rect::new(
                    pause_button_pos.x,
                    pause_button_pos.y,
                    pause_button_size.x,
                    pause_button_size.y,
                ),
                "Pause".to_string(),
                20,
                pause_button_kind,
            ),
            pause_overlay: OverlayV1::new(
                "Paused".to_string(),
                None,
                "Resume".to_string(),
                "Return to Home".to_string(),
            ),
            ship: Ship::new(ship_bounds, ship_kind),
            asteroid_field: AsteroidField::new(
                asteroid_count,
                asteroid_kind,
                asteroid_scale_limits,
            ),
            score: 0,
            time_remaining,
            hud: Hud::new(),
        }
    }

    /// Points earned by destroying asteroids so far this run.
    pub fn score(&self) -> u32 {
        self.score
    }

    fn set_outcome(&mut self, outcome: GameOutcome) {
        let title = match outcome {
            GameOutcome::Victory => "Victory!",
            GameOutcome::Defeat => "Game Over",
        };
        self.end_overlay = Some(OverlayV1::new(
            title.to_string(),
            Some(format!("Score: {}", self.score)),
            "Retry".to_string(),
            "Return to Home".to_string(),
        ));
        self.outcome = Some(outcome);
        println!("{LOG_PREFIX} Game ended: {outcome:?}");

        // play the matching outcome sound
        let outcome_sound = match outcome {
            GameOutcome::Victory => GameOutcomeSound::Victory,
            GameOutcome::Defeat => GameOutcomeSound::Defeat,
        };
        audio::play_sound_once(
            AudioRepository::get_instance()
                .game_outcome_sounds
                .get_clip_for(&outcome_sound)
                .unwrap_or_else(|| panic!("{LOG_PREFIX} outcome sound not loaded")),
        );
    }
}

impl Drawable for GameplayPage {
    fn draw(&self) {
        self.pause_button.draw();
        self.ship.draw();
        self.asteroid_field.draw();
        self.hud.draw();
        if let Some(overlay) = &self.end_overlay {
            overlay.draw();
        } else if self.paused {
            self.pause_overlay.draw();
        }
    }
}

impl StateUpdatable<()> for GameplayPage {
    fn update_state(&mut self, _args: ()) {
        if let Some(overlay) = &mut self.end_overlay {
            overlay.update_state(());

            // Keep animating the destroyed ship so its explosion plays out;
            // the player can't move or shoot once it's dead either way.
            if matches!(self.outcome, Some(GameOutcome::Defeat)) {
                self.ship.update_state(());
            }
        } else if self.paused {
            self.pause_overlay.update_state(());
        } else {
            self.pause_button.update_state(());
            self.ship.update_state(());
            self.asteroid_field.update_state(());

            // Resolve ship <-> asteroid collisions here; ship and asteroid field entities remain independent of each other.
            if self.ship.is_alive() {
                let damage = self.asteroid_field.resolve_collision(&self.ship);
                if damage > 0 {
                    self.ship.take_damage(damage);
                    println!("{LOG_PREFIX} ship took {damage} damage");
                    if !self.ship.is_alive() {
                        println!("{LOG_PREFIX} ship destroyed");
                    }
                }
            }

            // Resolve bullet <-> asteroid collisions
            // Note: bullets already in flight keep hitting asteroids
            // even after the ship that fired them has died.
            self.ship
                .resolve_bullet_collisions(&mut self.asteroid_field, &mut self.score);

            self.time_remaining = (self.time_remaining - get_frame_time()).max(0.0);

            self.hud.update_state(HudData {
                score: self.score,
                cur_health: self.ship.cur_health(),
                max_health: self.ship.max_health(),
                time_remaining: self.time_remaining,
            });

            if !self.ship.is_alive() {
                self.set_outcome(GameOutcome::Defeat);
            } else if self.time_remaining <= 0.0 {
                self.set_outcome(GameOutcome::Victory);
            }
        }
    }
}

impl Interactive for GameplayPage {
    type Event = Option<GamplayPageEvent>;
    fn poll_event(&self) -> Self::Event {
        if let Some(overlay) = &self.end_overlay {
            return match overlay.poll_event() {
                Some(OverlayV1Event::Option1Clicked) => Some(GamplayPageEvent::RetryButtonPressed),
                Some(OverlayV1Event::Option2Clicked) => {
                    Some(GamplayPageEvent::ReturnToHomeButtonPressed)
                }
                None => None,
            };
        }
        if self.paused {
            return match self.pause_overlay.poll_event() {
                Some(OverlayV1Event::Option1Clicked) => Some(GamplayPageEvent::ResumeButtonPressed),
                Some(OverlayV1Event::Option2Clicked) => {
                    Some(GamplayPageEvent::ReturnToHomeButtonPressed)
                }
                None => None,
            };
        }
        if let Some(ButtonEvents::Clicked) = self.pause_button.poll_event() {
            return Some(GamplayPageEvent::PauseButtonPressed);
        }
        None
    }
}

impl SelfEventHandler for GameplayPage {
    fn handle_self_event(&mut self, _event: Self::Event) {
        if let Some(overlay) = &mut self.end_overlay {
            let event = overlay.poll_event();
            overlay.handle_self_event(event);
        } else if self.paused {
            let event = self.pause_overlay.poll_event();

            self.pause_overlay.handle_self_event(event);

            if matches!(event, Some(OverlayV1Event::Option1Clicked)) {
                // Option 1: Resume button was clicked
                self.paused = false;
            }
        } else {
            let event = self.pause_button.poll_event();

            self.pause_button.handle_self_event(event);

            if matches!(event, Some(ButtonEvents::Clicked)) {
                self.paused = true;
            }
        }
    }
}
