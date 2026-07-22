pub mod utils;
pub mod object;
pub mod rendering;
pub mod background;

use macroquad::prelude::*;
use crate::game::rendering::{Drawable, StateUpdatable};

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
            window_title: "Space Shooter Classic".to_owned(),
            window_width: 605,
            window_height: 455,
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