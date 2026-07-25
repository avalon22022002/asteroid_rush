use asteroid_rush::game::{
    self,
    interaction::{Interactive, SelfEventHandler},
    rendering::{Drawable, StateUpdatable},
    ui::pages::home_page::{HomePage, HomePageEvent},
    window_conf,
};
use macroquad::prelude::*;

#[macroquad::main(window_conf)]
async fn main() {
    println!("Moving star field with Home Page!");

    let mut game = game::Game::new();
    let mut home_page = HomePage::new();
    game::audio::load_sounds().await;

    loop {
        game.update_state(());
        game.draw();

        home_page.update_state(());
        home_page.draw();
        let home_page_event = home_page.poll_event();
        match home_page_event {
            Some(HomePageEvent::NewGame) => {
                println!("New Game clicked");
            }
            Some(HomePageEvent::Exit) => {
                std::process::exit(0);
            }
            None => {}
        }
        home_page.handle_self_event(home_page_event);
        next_frame().await;
    }
}
