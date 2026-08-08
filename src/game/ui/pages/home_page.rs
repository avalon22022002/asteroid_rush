use macroquad::prelude::*;

use crate::game::{
    BASE_HEIGHT, BASE_WIDTH,
    entities::ship::{Ship, ShipKind},
    interaction::{Interactive, SelfEventHandler},
    rendering::{Drawable, StateUpdatable},
    ui::components::{
        banner::{Banner, BannerKind},
        button::{Button, ButtonEvents},
    },
};

/// Choices the player can make from the title screen.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HomePageEvent {
    NewGame,
    Exit,
}

pub struct HomePage {
    main_banner: Banner,
    new_game_button: Button,
    exit_button: Button,
    ship: Ship,
}

impl Default for HomePage {
    fn default() -> Self {
        Self::new()
    }
}

impl HomePage {
    pub fn new() -> Self {
        let button_width = BASE_WIDTH * 0.5;
        let button_height = BASE_HEIGHT * 0.2;
        let gap = 10.0;
        let x = 20.0;
        let new_game_y = BASE_HEIGHT * 0.40;
        let exit_y = new_game_y + button_height + gap;

        Self {
            main_banner: Banner::new(
                BannerKind::HomePageMain,
                Vec2::new(x, 50.0),
                Vec2::new(450.0, 111.0),
                None,
            ),
            new_game_button: Button::new(
                Rect::new(x, new_game_y, button_width, button_height),
                "New Game".to_string(),
                30,
            ),
            exit_button: Button::new(
                Rect::new(x, exit_y, button_width, button_height),
                "Exit".to_string(),
                30,
            ),
            ship: Ship::new(
                Vec2::new(BASE_WIDTH - 300.0, 180.0),
                ShipKind::Sentinel,
                "Sentinel".to_string(),
            ),
        }
    }
}

impl Drawable for HomePage {
    fn draw(&self) {
        self.main_banner.draw();
        self.new_game_button.draw();
        self.exit_button.draw();
        self.ship.draw();
    }
}

impl StateUpdatable<()> for HomePage {
    fn update_state(&mut self, _args: ()) {
        self.new_game_button.update_state(());
        self.exit_button.update_state(());
        self.ship.update_state(());
    }
}

impl Interactive for HomePage {
    type Event = Option<HomePageEvent>;

    /// Checks both buttons for a click this frame. If both somehow fire on
    /// the same frame, New Game takes priority over Exit.
    fn poll_event(&self) -> Self::Event {
        if let Some(ButtonEvents::Clicked) = self.new_game_button.poll_event() {
            return Some(HomePageEvent::NewGame);
        }
        if let Some(ButtonEvents::Clicked) = self.exit_button.poll_event() {
            return Some(HomePageEvent::Exit);
        }
        None
    }
}

impl SelfEventHandler for HomePage {
    /// Fans out to each button's own self-contained feedback (e.g. its click
    /// sound) — the same delegation `draw`/`update_state` already do above.
    /// `HomePageEvent` (the `event` param) is app-level and means nothing to
    /// a `Button`, so it's unused here: each child instead re-derives and
    /// handles *its own* event, independently of any priority ordering
    /// `poll_event` applies when deciding what the click means for the page.
    fn handle_self_event(&mut self, _event: Self::Event) {
        self.new_game_button
            .handle_self_event(self.new_game_button.poll_event());
        self.exit_button
            .handle_self_event(self.exit_button.poll_event());
    }
}
