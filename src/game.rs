use macroquad::prelude::*;
pub mod utils;
pub mod game_object;
pub mod rendering;
pub mod background;

pub fn game_window_conf() -> Conf {
    Conf {
        window_title: "Space Shooter Classic".to_owned(),
        window_width: 605,
        window_height: 455,
        ..Default::default()
    }
}