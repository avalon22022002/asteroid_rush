use macroquad::prelude::*;
use asteroid_rush::game::{
   self,
   window_conf,
   interaction::Interactive,
   rendering::{Drawable, StateUpdatable}, 
   ui::button::Events,
   object::{HasId}
};

#[macroquad::main(window_conf)]
async fn main() {
    println!("Moving star field!");
    

    let mut game= game::Game::new();
    let mut b=game::ui::button::Button::new(
      Rect::new(20.0,20.0,100.0,50.0),
      "Click Me".to_string(), 
      BLUE, 
      30
   );
   
     loop {  
        game.draw();
        game.update_state(());
   
        b.draw();
        b.update_state(());
        match b.poll_event() {
            Some(event) => {
               if event == Events::Clicked {
                  println!("Button With Id {} is clicked", b.id())
               }
            },
            None => {},
        }

        next_frame().await;
     }
}
