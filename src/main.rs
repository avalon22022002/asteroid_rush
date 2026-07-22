use asteroid_rush::game::{self, game_window_conf, rendering::{Drawable, StateUpdatable}};
use macroquad::prelude::*;

#[macroquad::main(game_window_conf)]
async fn main() {
    println!("Moving star field!");

    let mut bg= game::background::Background::new();
     loop {  
        bg.draw();
        bg.update_state(());

        next_frame().await;
     }
}
