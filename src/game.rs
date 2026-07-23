pub mod utils;
pub mod object;
pub mod rendering;
pub mod interaction;
pub mod background;
pub mod ui;

use macroquad::prelude::*;
use crate::game::rendering::{Drawable, StateUpdatable};

/// Base window size the game is designed at. UI elements scale their visuals
/// relative to this so proportions hold up if the window is resized.
pub const BASE_WIDTH: f32 = 605.0;
pub const BASE_HEIGHT: f32 = 455.0;

pub struct Game {
    background: background::Background,
}

impl Game {
    pub fn new()->Self{
        Self{
            background: background::Background::new()
        }
    }

    fn window_conf()-> Conf {
         Conf {
            window_title: "AstroRush: Space Shooter Classic".to_owned(),
            window_width: BASE_WIDTH as i32,
            window_height: BASE_HEIGHT as i32,
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

impl StateUpdatable<()> for Game{
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