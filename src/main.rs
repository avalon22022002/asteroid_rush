use asteroid_rush::game::{
    self,
    asset_repository::traits::Singleton,
    traits::interaction::{Interactive, SelfEventHandler},
    traits::rendering::{Drawable, StateUpdatable},
    ui::page_manager::{ PageManager},
    window_conf,
};
use macroquad::prelude::*;

#[macroquad::main(window_conf)]
async fn main() {
    println!("Moving star field with Home Page!");

    // Both repositories build (and, via Singleton::new, fully load) on this
    // first `get_instance` call, so every sound/texture is decoded here,
    // up front, instead of stalling whichever page first needs one.
    game::asset_repository::audio_repository::AudioRepository::get_instance();
    game::asset_repository::sprite_repository::SpriteRepository::get_instance();

    let mut game = game::Game::new();
    let mut page_manager = PageManager::new();

    loop {
        game.update_state(());
        game.draw();

        page_manager.update_state(());
        page_manager.draw();
        let page_event = page_manager.poll_event();
        page_manager.handle_self_event(page_event);
        next_frame().await;
    }
}
