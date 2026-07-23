use macroquad::prelude::*;
use asteroid_rush::game::{
   self,
   window_conf,
   interaction::Interactive,
   rendering::{Drawable, StateUpdatable},
   ui::pages::home_page::{HomePage, HomePageEvent},
};

#[macroquad::main(window_conf)]
async fn main() {
    println!("Moving star field!");


    let mut game= game::Game::new();
    let mut home_page = HomePage::new();

     loop {
        game.update_state(());
        game.draw();

        home_page.update_state(());
        home_page.draw();
        match home_page.poll_event() {
            Some(HomePageEvent::NewGame) => {
               println!("New Game clicked");
            }
            Some(HomePageEvent::Exit) => {
               std::process::exit(0);
            }
            None => {}
        }

        next_frame().await;
     }
}
