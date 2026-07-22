use asteroid_rush::game::{self, window_conf, rendering::{Drawable, StateUpdatable}};
use macroquad::prelude::*;

#[macroquad::main(window_conf)]
async fn main() {
    println!("Moving star field!");
    

    let mut game= game::Game::new();
     loop {  
        game.draw();
        game.update_state(());

        next_frame().await;
     }
}
