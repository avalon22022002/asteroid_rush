use macroquad::prelude::*;

use crate::game::{
    BASE_HEIGHT, BASE_WIDTH,
    asset_repository::sprite_repository::traits::SpriteBounds,
    game_config::GameConfig,
    traits::{
        interaction::{Interactive, SelfEventHandler},
        rendering::{Drawable, StateUpdatable},
    },
    ui::components::button::{Button, ButtonEvents, ButtonKind},
    utils::aspect_size_from_fixed_width,
};

/// Lines of the "how to play" body text, drawn top-to-bottom.
const INSTRUCTIONS: &[&str] = &[
    "Arrow keys / WASD - move your ship",
    "Space - fire",
    "Score points by hitting asteroids",
    "Survive until the timer runs out!",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BriefingPageEvent {
    StartGame,
    BackButtonPressed,
}

/// Sits between level selection and gameplay: a brief "how to play" recap for
/// the chosen level, with a button to launch into `GameplayPage`.
pub struct BriefingPage {
    game_config: GameConfig,
    start_button: Button,
    back_button: Button,
}

impl BriefingPage {
    pub fn new(game_config: GameConfig) -> Self {
        let button_kind = ButtonKind::Basic;

        let start_button_size =
            aspect_size_from_fixed_width(BASE_WIDTH * 0.28, button_kind.aspect_ratio());
        let start_button_pos =
            Vec2::new((BASE_WIDTH - start_button_size.x) / 2.0, BASE_HEIGHT * 0.7);

        let back_button_size = aspect_size_from_fixed_width(90.0, button_kind.aspect_ratio());
        let back_button_pos = Vec2::new(BASE_WIDTH - back_button_size.x - 20.0, 15.0);

        Self {
            game_config,
            start_button: Button::new(
                Rect::new(
                    start_button_pos.x,
                    start_button_pos.y,
                    start_button_size.x,
                    start_button_size.y,
                ),
                "Start Game".to_string(),
                28,
                button_kind,
            ),
            back_button: Button::new(
                Rect::new(
                    back_button_pos.x,
                    back_button_pos.y,
                    back_button_size.x,
                    back_button_size.y,
                ),
                "Back".to_string(),
                20,
                button_kind,
            ),
        }
    }

    pub fn game_config(&self) -> &GameConfig {
        &self.game_config
    }

    /// Draws `text` horizontally centered at `baseline_y`.
    fn draw_text_centered(text: &str, baseline_y: f32, font_size: u16, color: Color) {
        let ts = measure_text(text, None, font_size, 1.0);
        draw_text_ex(
            text,
            (BASE_WIDTH - ts.width) / 2.0,
            baseline_y,
            TextParams {
                font_size,
                color,
                ..Default::default()
            },
        );
    }
}

impl Drawable for BriefingPage {
    fn draw(&self) {
        Self::draw_text_centered(
            "MISSION BRIEFING",
            BASE_HEIGHT * 0.16,
            40,
            Color::new(0.4, 0.7, 1.0, 1.0),
        );
        Self::draw_text_centered(
            &format!("Level {:?}", self.game_config.level()),
            BASE_HEIGHT * 0.24,
            22,
            Color::new(1.0, 0.85, 0.4, 1.0),
        );

        let instructions_top = BASE_HEIGHT * 0.36;
        let line_gap = 34.0;
        for (i, line) in INSTRUCTIONS.iter().enumerate() {
            Self::draw_text_centered(line, instructions_top + i as f32 * line_gap, 22, WHITE);
        }

        // Draw Buttons
        self.start_button.draw();
        self.back_button.draw();
    }
}

impl StateUpdatable<()> for BriefingPage {
    fn update_state(&mut self, _args: ()) {
        self.start_button.update_state(());
        self.back_button.update_state(());
    }
}

impl Interactive for BriefingPage {
    type Event = Option<BriefingPageEvent>;

    fn poll_event(&self) -> Self::Event {
        if let Some(ButtonEvents::Clicked) = self.start_button.poll_event() {
            return Some(BriefingPageEvent::StartGame);
        }
        if let Some(ButtonEvents::Clicked) = self.back_button.poll_event() {
            return Some(BriefingPageEvent::BackButtonPressed);
        }
        None
    }
}

impl SelfEventHandler for BriefingPage {
    fn handle_self_event(&mut self, _event: Self::Event) {
        self.start_button
            .handle_self_event(self.start_button.poll_event());
        self.back_button
            .handle_self_event(self.back_button.poll_event());
    }
}
