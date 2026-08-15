use macroquad::prelude::*;

use crate::game::{
    BASE_HEIGHT, BASE_WIDTH,
    asset_repository::sprite_repository::traits::SpriteBounds,
    entities::ship::{Ship, ShipKind},
    interaction::{Interactive, SelfEventHandler},
    rendering::{Drawable, StateUpdatable},
    ui::components::{
        banner::{Banner, BannerKind},
        button::{Button, ButtonEvents, ButtonKind},
    },
    utils::{aspect_size_from_fixed_height, aspect_size_from_fixed_width},
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

        // HomePage main banner config
        let banner_kind = BannerKind::HomeTitle;
        let banner_pos = Vec2::new(40.0, BASE_HEIGHT * 0.09);
        let banner_size = aspect_size_from_fixed_height(BASE_HEIGHT * 0.24, banner_kind.aspect_ratio());

        // HomePage button common config
        let button_kind = ButtonKind::Basic;
        let button_size = aspect_size_from_fixed_width(BASE_WIDTH * 0.3, button_kind.aspect_ratio());
        let button_pos_x = banner_pos.x + (banner_size.x - button_size.x) / 2.0; // Center the buttons horizontally within the banner's span.
        let button_vertical_gap = 14.0;
        
        // HomePage button specific config
        let new_game_button_y = BASE_HEIGHT * 0.42;
        let exit_button_y = new_game_button_y + button_size.y + button_vertical_gap;

        Self {
            main_banner: Banner::new(banner_kind, banner_pos, banner_size),
            new_game_button: Button::new(
                Rect::new(button_pos_x, new_game_button_y, button_size.x, button_size.y),
                "New Game".to_string(),
                34,
                button_kind,
            ),
            exit_button: Button::new(
                Rect::new(button_pos_x, exit_button_y, button_size.x, button_size.y),
                "Exit".to_string(),
                34,
                button_kind,
            ),
            ship: Ship::new(
                // Right-anchored, vertically aligned with the button group.
                Vec2::new(BASE_WIDTH - 380.0, BASE_HEIGHT * 0.36),
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
    /// Lets each button react to its own event (e.g. play its click sound),
    /// the same way `HomePage` delegates drawing and state updates to its
    /// children elsewhere. `_event` is this page's own `HomePageEvent`
    /// verdict, which means nothing to a `Button`, so it's ignored here —
    /// each child re-polls and handles its own event instead.
    fn handle_self_event(&mut self, _event: Self::Event) {
        self.new_game_button
            .handle_self_event(self.new_game_button.poll_event());
        self.exit_button
            .handle_self_event(self.exit_button.poll_event());
    }
}
