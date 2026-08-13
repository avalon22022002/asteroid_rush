use macroquad::prelude::*;

use crate::game::{
    BASE_HEIGHT, BASE_WIDTH,
    asset_repository::sprite_repository::{traits::SpriteBounds, BannerV1Textures},
    entities::asteroidfield::AsteroidField,
    interaction::{Interactive, SelfEventHandler},
    rendering::{Drawable, StateUpdatable},
    ui::components::{
        banner::Banner,
        button::{Button, ButtonEvents, ButtonKind},
    },
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LevelSelectionPageEvent {
    Level1Selected,
    Level2Selected,
    Level3Selected,
    /// The player pressed the back button to return to the previous page
    /// (the ship selection page).
    BackButtonPressed,
}

pub struct LevelSelectionPage {
    main_banner: Banner,
    level_1_button: Button,
    level_2_button: Button,
    level_3_button: Button,
    back_button: Button,
    // TODO: selecting each level should change the speed of asteroids
    asteroid_field: AsteroidField,
}

impl Default for LevelSelectionPage {
    fn default() -> Self {
        Self::new()
    }
}

impl LevelSelectionPage {
    pub fn new() -> Self {
        let button_width = BASE_WIDTH * 0.33;
        // Derive height from the button art's width:height so it isn't stretched.
        let button_height = button_width / ButtonKind::Basic.aspect_ratio();
        let gap = 10.0;
        let x = 20.0;
        let level_1_y = BASE_HEIGHT * 0.35;
        let level_2_y = level_1_y + button_height + gap;
        let level_3_y = level_2_y + button_height + gap;

        // Small back button in the top-right corner — the banner already
        // occupies the top-left. Height derives from the art's aspect ratio
        // so the small size isn't stretched.
        let back_width = 90.0;
        let back_height = back_width / ButtonKind::Basic.aspect_ratio();

        Self {
            main_banner: Banner::new(
                BannerV1Textures::LevelSelectionPageMain,
                Vec2::new(x, 25.0),
                Vec2::new(450.0, 111.0),
            ),
            level_1_button: Button::new(
                Rect::new(x, level_1_y, button_width, button_height),
                "Level 1".to_string(),
                30,
                ButtonKind::Basic,
            ),
            level_2_button: Button::new(
                Rect::new(x, level_2_y, button_width, button_height),
                "Level 2".to_string(),
                30,
                ButtonKind::Basic,
            ),
            level_3_button: Button::new(
                Rect::new(x, level_3_y, button_width, button_height),
                "Level 3".to_string(),
                30,
                ButtonKind::Basic,
            ),
            back_button: Button::new(
                Rect::new(BASE_WIDTH - back_width - 20.0, 15.0, back_width, back_height),
                "Back".to_string(),
                20,
                ButtonKind::Basic,
            ),
            asteroid_field: AsteroidField::new(15, None),
        }
    }
}

impl Drawable for LevelSelectionPage {
    fn draw(&self) {
        self.asteroid_field.draw();
        self.main_banner.draw();
        self.level_1_button.draw();
        self.level_2_button.draw();
        self.level_3_button.draw();
        self.back_button.draw();
    }
}

impl StateUpdatable<()> for LevelSelectionPage {
    fn update_state(&mut self, _args: ()) {
        self.asteroid_field.update_state(());
        self.level_1_button.update_state(());
        self.level_2_button.update_state(());
        self.level_3_button.update_state(());
        self.back_button.update_state(());
    }
}

impl Interactive for LevelSelectionPage {
    type Event = Option<LevelSelectionPageEvent>;

    /// Checks all three buttons for a click this frame. If more than one
    /// somehow fires on the same frame, the lowest level number wins.
    fn poll_event(&self) -> Self::Event {
        if let Some(ButtonEvents::Clicked) = self.back_button.poll_event() {
            return Some(LevelSelectionPageEvent::BackButtonPressed);
        }
        if let Some(ButtonEvents::Clicked) = self.level_1_button.poll_event() {
            return Some(LevelSelectionPageEvent::Level1Selected);
        }
        if let Some(ButtonEvents::Clicked) = self.level_2_button.poll_event() {
            return Some(LevelSelectionPageEvent::Level2Selected);
        }
        if let Some(ButtonEvents::Clicked) = self.level_3_button.poll_event() {
            return Some(LevelSelectionPageEvent::Level3Selected);
        }
        None
    }
}

impl SelfEventHandler for LevelSelectionPage {
    fn handle_self_event(&mut self, _event: Self::Event) {
        self.level_1_button
            .handle_self_event(self.level_1_button.poll_event());
        self.level_2_button
            .handle_self_event(self.level_2_button.poll_event());
        self.level_3_button
            .handle_self_event(self.level_3_button.poll_event());
        self.back_button
            .handle_self_event(self.back_button.poll_event());
    }
}
