pub mod animation;
pub mod asset_repository;
pub mod background;
pub mod blink;
pub mod entities;
pub mod game_config;
pub mod traits;
pub mod ui;
pub mod utils;

use crate::game::traits::rendering::{Drawable, StateUpdatable};
use macroquad::prelude::*;

/// Game window reference width, in logical pixels.
pub const BASE_WIDTH: f32 = 960.0;
/// Game window reference height, in logical pixels.
pub const BASE_HEIGHT: f32 = 540.0;

pub struct Game {
    background: background::Background,
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

impl Game {
    pub fn new() -> Self {
        Self {
            background: background::Background::new(),
        }
    }

    fn window_conf() -> Conf {
        Conf {
            window_title: "AstroRush: Space Shooter Classic".to_owned(),
            window_width: BASE_WIDTH as i32,
            window_height: BASE_HEIGHT as i32,
            // Lock the window size: a fixed viewport keeps component positions
            // and scales simple, since everything is laid out against a known
            // BASE_WIDTH x BASE_HEIGHT and never has to reflow on resize.
            window_resizable: false,
            ..Default::default()
        }
    }
}

impl Drawable for Game {
    fn draw(&self) {
        // draw background
        self.background.draw();
    }
}

impl StateUpdatable<()> for Game {
    fn update_state(&mut self, data: ()) {
        // update background state
        self.background.update_state(data);
    }
}

/// Free function required by `#[macroquad::main(window_conf)]` — the macro expects
/// a top-level function it can call directly, so this just delegates to `Game::window_conf()`.
pub fn window_conf() -> Conf {
    Game::window_conf()
}
