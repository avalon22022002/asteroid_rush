use macroquad::prelude::*;

use crate::game::{
    BASE_HEIGHT, BASE_WIDTH,
    asset_repository::{
        sprite_repository::{traits::SpriteTextures, SpriteRepository, ShipV1Textures, BannerV1Textures, ButtonV1Textures},
        traits::Singleton,
    },
    entities::{animation::Animation, ship::ShipKind},
    interaction::{Interactive, SelfEventHandler},
    rendering::{Drawable, StateUpdatable},
    ui::components::{
        banner::Banner,
        button::ButtonEvents,
        icon_label_button::IconLabelButton,
    },
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShipSelectionPageEvent {
    ShipSelected(ShipKind),
}

pub struct ShipSelectionPage {
    main_banner: Banner,
    vanguard_button: IconLabelButton,
    sentinel_button: IconLabelButton,
    viper_button: IconLabelButton,
}

impl Default for ShipSelectionPage {
    fn default() -> Self {
        Self::new()
    }
}

impl ShipSelectionPage {
    pub fn new() -> Self {
        let button_width = BASE_WIDTH * 0.42;
        // Derive height from the button art's own width:height so it isn't stretched.
        let button_height = button_width / ButtonV1Textures::BasicScifiV1.aspect_ratio();
        // Button trims the texture's transparent margins, so a button's bounds
        // are exactly its visible button art — this is a true gap between them.
        let gap = 12.0;
        let x = 20.0;
        let vanguard_y = BASE_HEIGHT * 0.28;
        let sentinel_y = vanguard_y + button_height + gap;
        let viper_y = sentinel_y + button_height + gap;
        let icon_size = Vec2::splat(button_height * 0.7);

        // Only Sentinel has dedicated ship art today — reused here as a
        // placeholder icon for Vanguard/Viper until theirs is added.
        let ship_sprites = &SpriteRepository::get_instance().ship_v1_sprite;
        let icon = || {
            Animation::new(
                ship_sprites.get_textures_for(&ShipV1Textures::SentinelAlive),
                icon_size,
                1.0,
            )
        };

        Self {
            main_banner: Banner::new(
                BannerV1Textures::ShipSelectionPageMain,
                Vec2::new(x, 10.0),
                Vec2::new(450.0, 111.0),
            ),
            vanguard_button: IconLabelButton::new(
                Rect::new(x, vanguard_y, button_width, button_height),
                icon(),
                ShipKind::Vanguard.display_name().to_string(),
                ShipKind::Vanguard.role().to_string(),
            ),
            sentinel_button: IconLabelButton::new(
                Rect::new(x, sentinel_y, button_width, button_height),
                icon(),
                ShipKind::Sentinel.display_name().to_string(),
                ShipKind::Sentinel.role().to_string(),
            ),
            viper_button: IconLabelButton::new(
                Rect::new(x, viper_y, button_width, button_height),
                icon(),
                ShipKind::Viper.display_name().to_string(),
                ShipKind::Viper.role().to_string(),
            ),
        }
    }
}

impl Drawable for ShipSelectionPage {
    fn draw(&self) {
        self.main_banner.draw();
        self.vanguard_button.draw();
        self.sentinel_button.draw();
        self.viper_button.draw();
    }
}

impl StateUpdatable<()> for ShipSelectionPage {
    fn update_state(&mut self, _args: ()) {
        self.vanguard_button.update_state(());
        self.sentinel_button.update_state(());
        self.viper_button.update_state(());
    }
}

impl Interactive for ShipSelectionPage {
    type Event = Option<ShipSelectionPageEvent>;

    /// Checks all three ship buttons for a click this frame. If more than
    /// one somehow fires on the same frame, Vanguard wins.
    fn poll_event(&self) -> Self::Event {
        if let Some(ButtonEvents::Clicked) = self.vanguard_button.poll_event() {
            return Some(ShipSelectionPageEvent::ShipSelected(ShipKind::Vanguard));
        }
        if let Some(ButtonEvents::Clicked) = self.sentinel_button.poll_event() {
            return Some(ShipSelectionPageEvent::ShipSelected(ShipKind::Sentinel));
        }
        if let Some(ButtonEvents::Clicked) = self.viper_button.poll_event() {
            return Some(ShipSelectionPageEvent::ShipSelected(ShipKind::Viper));
        }
        None
    }
}

impl SelfEventHandler for ShipSelectionPage {
    fn handle_self_event(&mut self, _event: Self::Event) {
        self.vanguard_button
            .handle_self_event(self.vanguard_button.poll_event());
        self.sentinel_button
            .handle_self_event(self.sentinel_button.poll_event());
        self.viper_button
            .handle_self_event(self.viper_button.poll_event());
    }
}
