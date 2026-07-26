use asteroid_rush::game::{
    self,
    interaction::{Interactive, SelfEventHandler},
    rendering::{Drawable, StateUpdatable},
    ui::pages::{PageEvents, PageManager, home_page::HomePageEvent},
    window_conf,
};
use macroquad::prelude::*;

#[macroquad::main(window_conf)]
async fn main() {
    println!("Moving star field with Home Page!");

    let mut game = game::Game::new();
    let mut page_manager = PageManager::new();
    game::audio::load_sounds().await;

    loop {
        game.update_state(());
        game.draw();

        page_manager.update_state(());
        page_manager.draw();
        let page_event = page_manager.poll_event();
        match page_event {
            Some(PageEvents::HomePageEvent(HomePageEvent::NewGame)) => {}
            Some(PageEvents::HomePageEvent(HomePageEvent::Exit)) => {
                std::process::exit(0);
            }
            _ => {}
        }
        page_manager.handle_self_event(page_event);
        next_frame().await;
    }
}
