use macroquad::prelude::*;

use crate::game::{
    interaction::Interactive,
    rendering::{Drawable, StateUpdatable},
    ui::components::button::{Button, Events as ButtonEvent},
    BASE_HEIGHT, BASE_WIDTH,
};

/// Choices the player can make from the title screen.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HomePageEvent {
    NewGame,
    Exit,
}

pub struct HomePage {
    new_game_button: Button,
    exit_button: Button,
}

impl HomePage {
    pub fn new() -> Self {
        let button_width = BASE_WIDTH * 0.46;
        let button_height = BASE_HEIGHT * 0.13;
        let gap = 10.0;
        let x = 20.0;
        let new_game_y = BASE_HEIGHT * 0.40;
        let exit_y = new_game_y + button_height + gap;

        Self {
            new_game_button: Button::new(
                Rect::new(x, new_game_y, button_width, button_height),
                "New Game".to_string(),
                BLUE,
                30,
                Some(GREEN),
            ),
            exit_button: Button::new(
                Rect::new(x, exit_y, button_width, button_height),
                "Exit".to_string(),
                BLUE,
                30,
                Some(RED),
            ),
        }
    }
}

impl Drawable for HomePage {
    fn draw(&self) {
        self.new_game_button.draw();
        self.exit_button.draw();
    }
}

impl StateUpdatable<()> for HomePage {
    fn update_state(&mut self, _args: ()) {
        self.new_game_button.update_state(());
        self.exit_button.update_state(());
    }
}

impl Interactive for HomePage {
    type Event = Option<HomePageEvent>;

    /// Checks both buttons for a click this frame. If both somehow fire on
    /// the same frame, New Game takes priority over Exit.
    fn poll_event(&self) -> Self::Event {
        if let Some(ButtonEvent::Clicked) = self.new_game_button.poll_event() {
            return Some(HomePageEvent::NewGame);
        }
        if let Some(ButtonEvent::Clicked) = self.exit_button.poll_event() {
            return Some(HomePageEvent::Exit);
        }
        None
    }
}
