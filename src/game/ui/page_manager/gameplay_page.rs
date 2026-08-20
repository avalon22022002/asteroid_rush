use crate::game::{
    game_config::{ GameConfig}, 
    rendering::{Drawable, StateUpdatable},
    ui::{
        components::button::Button
    },
    interaction::{Interactive, SelfEventHandler}
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GamplayPageEvent {
    PauseButtonPressed
}

pub struct GameplayPage {
    game_config: GameConfig,
    pause: Button,
}

impl GameplayPage{
    pub fn new(game_config: GameConfig)->Self{
        Self { game_config, pause: Button::default() }
    }
}

impl Drawable for GameplayPage {
    fn draw(&self) {
    }
}

impl StateUpdatable<()> for GameplayPage {
    fn update_state(&mut self, data: ()) {
        
    }
}

impl Interactive for GameplayPage {
    type Event = Option<GamplayPageEvent>;
    fn poll_event(&self) -> Self::Event {
        return None;
    }
}

impl SelfEventHandler for GameplayPage {
    fn handle_self_event(&mut self, event: Self::Event) {
        
    }
}