use asteroid_rush::game::{self, window_conf, rendering::{Drawable, StateUpdatable}};
use macroquad::prelude::*;

#[macroquad::main(window_conf)]
async fn main() {
    println!("Moving star field!");
    

    let mut game= game::Game::new();
    let mut b=game::ui::button::Button::new(
      Rect::new(20.0,20.0,100.0,50.0),
      "Click Me".to_string(), 
      BLUE, 
      30,
      Some(Box::new(|| {println!("Clicked");}))
   );
     loop {  
        game.draw();
        game.update_state(());
   
        b.draw();
        b.update_state(());

        next_frame().await;
     }
}
