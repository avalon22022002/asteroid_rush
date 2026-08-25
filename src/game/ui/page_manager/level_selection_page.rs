use macroquad::prelude::*;

use crate::game::{
    BASE_HEIGHT, BASE_WIDTH,
    asset_repository::{ traits::Singleton, sprite_repository::{traits::{SpriteBounds,SpriteTextures}, SpriteRepository}},
    animation::Animation,
    entities::asteroidfield::{AsteroidField, asteroid::AsteroidKind},
    interaction::{Interactive, SelfEventHandler}, rendering::{Drawable, StateUpdatable},
    ui::components::{
        banner::{Banner, BannerKind},
        button::{Button, ButtonEvents, ButtonKind},
        icon_label_button::{IconLabelButton},
        preview_card::{PreviewCard,PreviewCardEvent}
    },
    game_config::GameLevel,
    utils::{MinMax, aspect_size_from_fixed_height, aspect_size_from_fixed_width},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LevelSelectionPageEvent {
    LevelConfirmed(GameLevel),
    BackButtonPressed,
}

pub struct LevelSelectionPage {
    main_banner: Banner,
    level_1_button: IconLabelButton,
    level_2_button: IconLabelButton,
    level_3_button: IconLabelButton,
    back_button: Button,
    preview_card: PreviewCard,
    selected_level: GameLevel,
    asteroid_field: AsteroidField,
}

impl Default for LevelSelectionPage {
    fn default() -> Self {
        Self::new()
    }
}

impl LevelSelectionPage {
    pub fn new() -> Self {
        // Common x co-ordinate for Banner and Buttons
        let common_pos_x = 24.0;

        // Title banner Config
        let banner_kind = BannerKind::LevelSelectionTitle;
        let banner_pos = Vec2::new(common_pos_x, 12.0);
        let banner_size = aspect_size_from_fixed_height(BASE_HEIGHT * 0.18, banner_kind.aspect_ratio());

        // Level Selection Page button common config
        let button_kind = ButtonKind::Basic;
        let button_size = aspect_size_from_fixed_width(BASE_WIDTH * 0.30, button_kind.aspect_ratio());
        let button_vertical_gap = 14.0; // Vertical Spacing between buttons
        let button_title_font_size = 28; 
        let button_subtitle_font_size = 20;
        let button_x = common_pos_x + (banner_size.x - button_size.x) / 2.0; // Adjust the button's x co-ordinate so the buttons sit directly below the banners's "SELECT LEVEL" title.
        let buttons_right = button_x + button_size.x; // Right edge of the button column, used to position the card beside it

        // Adjust each individual button's y coordinate so they're stacked with a vertical gap between them.
        let level_1_button_y = banner_pos.y + banner_size.y + button_vertical_gap; // First button is below the banner
        let level_2_button_y = level_1_button_y + button_size.y + button_vertical_gap;
        let level_3_button_y = level_2_button_y + button_size.y + button_vertical_gap;

         // Adjust back button position and size at the top right corner of page
        let back_button_size = aspect_size_from_fixed_width(90.0, button_kind.aspect_ratio());
        let back_button_pos = Vec2::new(BASE_WIDTH - back_button_size.x - 20.0, 15.0);

        // Level Selection Page preview card config
        // Adjust the preview card so its centered in the right space beside the buttons column
        let preview_card_size = Vec2::new(320.0, 410.0);
        let preview_card_pos = Vec2::new(buttons_right + ((BASE_WIDTH - buttons_right) - preview_card_size.x) / 2.0 + 40.0, 100.0);
        let default_asteroid_kind = AsteroidKind::MoltenDarkAsteroid; // The preview opens on the first so the card is never empty.

        // Only Molten Asteroid has art today; reuse it as a placeholder for every asteroid level for now
        let asteroid_sprites = &SpriteRepository::get_instance().asteroid_v1_sprite;
        let asteroid_icon_size = Vec2::splat(button_size.y * 0.5);
        let default_asteroid_icon_textures = default_asteroid_kind.texture_kind();
        let default_asteroid_icon = || {
            Animation::new(
                asteroid_sprites.get_textures_for(&default_asteroid_icon_textures),
                asteroid_icon_size,
                12.0,
                Some(default_asteroid_icon_textures.content_bounds()), // Crop the asteroid art's transparent padding, so it fills its icon/panel
            )
        };

        Self {
            main_banner: Banner::new(
                banner_kind,
                banner_pos,
                banner_size,
            ),
            level_1_button: IconLabelButton::new(
                Rect::new(button_x, level_1_button_y, button_size.x, button_size.y),
                default_asteroid_icon(),
                "Level 1".to_string(),
                "Beginner".to_string(),
                button_title_font_size,
                button_subtitle_font_size,
            ),
            level_2_button: IconLabelButton::new(
                Rect::new(button_x, level_2_button_y, button_size.x, button_size.y),
                default_asteroid_icon(),
                "Level 2".to_string(),
                "Intermediate".to_string(),
                button_title_font_size,
                button_subtitle_font_size,
            ),
            level_3_button: IconLabelButton::new(
                Rect::new(button_x, level_3_button_y, button_size.x, button_size.y),
                default_asteroid_icon(),
                "Level 3".to_string(),
                "Advanced".to_string(),
                button_title_font_size,
                button_subtitle_font_size,
            ),
            back_button: Button::new(
                Rect::new(back_button_pos.x, back_button_pos.y, back_button_size.x, back_button_size.y),
                "Back".to_string(),
                20,
                ButtonKind::Basic,
            ),
            selected_level: GameLevel::Level1,
            preview_card: PreviewCard::new(
                Rect::new(preview_card_pos.x, preview_card_pos.y, preview_card_size.x, preview_card_size.y),
                default_asteroid_icon(),
                default_asteroid_kind.display_name().to_string(),
                default_asteroid_kind.difficulty_label().to_string(),
                default_asteroid_kind.preview_stats(),
                "SELECT LEVEL".to_string(),
            ),
            asteroid_field: AsteroidField::new(15, AsteroidKind::MoltenDarkAsteroid, MinMax{min: 0.3, max: 2.0}),
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
        self.preview_card.draw();
    }
}

impl StateUpdatable<()> for LevelSelectionPage {
    fn update_state(&mut self, _args: ()) {
        self.asteroid_field.update_state(());
        self.level_1_button.update_state(());
        self.level_2_button.update_state(());
        self.level_3_button.update_state(());
        self.back_button.update_state(());
        self.preview_card.update_state(());
    }
}

impl Interactive for LevelSelectionPage {
    type Event = Option<LevelSelectionPageEvent>;

    /// Checks all three buttons for a click this frame. If more than one
    /// somehow fires on the same frame, the lowest level number wins.
    fn poll_event(&self) -> Self::Event {
        if let Some(PreviewCardEvent::ActionButtonClicked) = self.preview_card.poll_event() {
            return Some(LevelSelectionPageEvent::LevelConfirmed(self.selected_level));
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
        self.preview_card
            .handle_self_event(self.preview_card.poll_event());

        // A level-button click pins that level's asteroid as the preview subject. The page
        // knows which button fired, so selection lives here, not on the buttons.
        let selected_level = if matches!(self.level_1_button.poll_event(), Some(ButtonEvents::Clicked)) {
            Some(GameLevel::Level1)
        } else if matches!(self.level_2_button.poll_event(), Some(ButtonEvents::Clicked)) {
            Some(GameLevel::Level2)
        } else if matches!(self.level_3_button.poll_event(), Some(ButtonEvents::Clicked)) {
            Some(GameLevel::Level3)
        } else {
            None
        };

        if let Some(level) = selected_level {
            self.selected_level = level;

            // Only Molten Dark asteroid has art today; reuse it as a placeholder for every level.
            let asteroid_sprite = &SpriteRepository::get_instance().asteroid_v1_sprite;
            let asteroid_kind=AsteroidKind::asteroid_kind_from_level(&self.selected_level);
            let asteroid_textures= asteroid_sprite.get_textures_for(&asteroid_kind.texture_kind());
            let asteroid_crop = Some(asteroid_kind.texture_kind().content_bounds());
            
            self.preview_card.set_content(
                Animation::new(
                    asteroid_textures,
                    Vec2::new(140.0, 178.0),
                    12.0,
                    asteroid_crop,
                ),
                asteroid_kind.display_name().to_string(),
                asteroid_kind.difficulty_label().to_string(),
                asteroid_kind.preview_stats(),
            );
        }
    }
}
