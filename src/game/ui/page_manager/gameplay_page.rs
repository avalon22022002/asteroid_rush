use macroquad::prelude::*;

use crate::game::{
    BASE_WIDTH,
    asset_repository::sprite_repository::traits::SpriteBounds,
    game_config::GameConfig,
    rendering::{Drawable, StateUpdatable},
    ui::components::{
        button::{Button, ButtonEvents, ButtonKind},
        overlay_v1::{OverlayV1, OverlayV1Event},
    },
    interaction::{Interactive, SelfEventHandler},
    utils::aspect_size_from_fixed_width,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GamplayPageEvent {
    PauseButtonPressed,
    ResumeButtonPressed,
    ReturnToHomeButtonPressed,
}

pub struct GameplayPage {
    game_config: GameConfig,
    /// Whether the game is currently paused.
    paused: bool,
    pause_button: Button,
    pause_overlay: OverlayV1,
}

impl GameplayPage {
    pub fn new(game_config: GameConfig) -> Self {
        // Pause Button
        let pause_button_kind = ButtonKind::Basic;
        let pause_button_size = aspect_size_from_fixed_width(90.0, pause_button_kind.aspect_ratio());
        let pause_button_pos = Vec2::new(BASE_WIDTH - pause_button_size.x - 20.0, 15.0);

        Self {
            game_config,
            paused: false,
            pause_button: Button::new(
                Rect::new(pause_button_pos.x, pause_button_pos.y, pause_button_size.x, pause_button_size.y),
                "Pause".to_string(),
                20,
                pause_button_kind,
            ),
            pause_overlay: OverlayV1::new(
                "Paused".to_string(),
                "Resume".to_string(),
                "Return to Home".to_string(),
            ),
        }
    }
}

impl Drawable for GameplayPage {
    fn draw(&self) {
        self.pause_button.draw();
        if self.paused {
            self.pause_overlay.draw();
        }
    }
}

impl StateUpdatable<()> for GameplayPage {
    fn update_state(&mut self, _args: ()) {
        if self.paused {
            self.pause_overlay.update_state(());
        } else {
            self.pause_button.update_state(());
        }
    }
}

impl Interactive for GameplayPage {
    type Event = Option<GamplayPageEvent>;
    fn poll_event(&self) -> Self::Event {
        if self.paused {
            return match self.pause_overlay.poll_event() {
                Some(OverlayV1Event::Option1Clicked) => Some(GamplayPageEvent::ResumeButtonPressed),
                Some(OverlayV1Event::Option2Clicked) => Some(GamplayPageEvent::ReturnToHomeButtonPressed),
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
        if self.paused {
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
