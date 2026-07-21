use asteroid_rush::game::{self, rendering::Drawable, game_window_conf};
use macroquad::prelude::*;

#[macroquad::main(game_window_conf)]
async fn main() {
    println!("stationary star field!"); // make it moving

    let bg= game::background::Background::new();
     loop {    
        bg.draw();

        next_frame().await;
     }
}
