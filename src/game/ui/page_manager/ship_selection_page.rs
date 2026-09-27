use macroquad::prelude::*;

use crate::game::{
    BASE_HEIGHT, BASE_WIDTH,
    asset_repository::{
        sprite_repository::{traits::{SpriteTextures, SpriteBounds}, SpriteRepository},
        traits::Singleton,
    },
    animation::Animation,
    entities::ship::{ship_kind::ShipKind, ship_stats::ShipStats},
    traits::interaction::{Interactive, SelfEventHandler},
    traits::rendering::{Drawable, StateUpdatable},
    ui::components::{
        banner::{Banner, BannerKind},
        button::{Button, ButtonEvents, ButtonKind},
        icon_label_button::IconLabelButton,
        preview_card::{PreviewCard, PreviewCardEvent},
    },
    utils::{aspect_size_from_fixed_height, aspect_size_from_fixed_width},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShipSelectionPageEvent {
    /// The player confirmed their selected ship via the preview card.
    ShipConfirmed(ShipKind),
    /// The player pressed the back button to return to the home page.
    BackButtonPressed,
}

pub struct ShipSelectionPage {
    main_banner: Banner,
    vanguard_button: IconLabelButton,
    sentinel_button: IconLabelButton,
    viper_button: IconLabelButton,
    back_button: Button,
    preview_card: PreviewCard,
    /// The kind of ship currently highlighted and shown in the preview card.
    selected_ship_kind: ShipKind,
}

impl Default for ShipSelectionPage {
    fn default() -> Self {
        Self::new()
    }
}

impl ShipSelectionPage {
    pub fn new() -> Self {
        // Common x co-ordinate for Banner and Buttons
        let common_pos_x = 24.0;

        // Title banner Config
        let banner_kind = BannerKind::ShipSelectionTitle;
        let banner_pos = Vec2::new(common_pos_x, 12.0);
        let banner_size = aspect_size_from_fixed_height(BASE_HEIGHT * 0.18, banner_kind.aspect_ratio());

        // Ship Selection Page button common config
        let button_kind = ButtonKind::Basic;
        let button_size = aspect_size_from_fixed_width(BASE_WIDTH * 0.30, button_kind.aspect_ratio());
        let button_vertical_gap = 14.0; // Vertical Spacing between buttons
        let button_title_font_size = 24;
        let button_subtitle_font_size = 16;
        let button_x = common_pos_x + (banner_size.x - button_size.x) / 2.0; // Adjust the button's x co-ordinate so the buttons sit directly below the banners's "SELECT SHIP" title.
        let buttons_right = button_x + button_size.x; // Right edge of the button column, used to position the card beside it

        // Adjust each individual button's y coordinate so they're stacked with a vertical gap between them.
        let vanguard_button_y = banner_pos.y + banner_size.y + button_vertical_gap; // First button is below the banner
        let sentinel_button_y = vanguard_button_y + button_size.y + button_vertical_gap;
        let viper_button_y = sentinel_button_y + button_size.y + button_vertical_gap;

        // Adjust back button position and size at the top right corner of page
        let back_button_size = aspect_size_from_fixed_width(90.0, button_kind.aspect_ratio());
        let back_button_pos = Vec2::new(BASE_WIDTH - back_button_size.x - 20.0, 15.0);

        // Ship Selection Page preview card config
        // Adjust the preview card so its centered in the right space beside the buttons column
        let preview_card_size = Vec2::new(320.0, 410.0);
        let preview_card_pos = Vec2::new(buttons_right + ((BASE_WIDTH - buttons_right) - preview_card_size.x) / 2.0 + 40.0, 100.0);
        let default_ship_kind = ShipKind::Vanguard; // The preview opens on the first so the card is never empty.

        let ship_sprites = &SpriteRepository::get_instance().ship_v1_sprite;
        let ship_icon_size = Vec2::splat(button_size.y * 0.7);
        
        let ship_icon_for = |kind: ShipKind| {
            let textures = kind.ship_v1_texture_kind();
            Animation::new(
                ship_sprites.get_textures_for(&textures),
                ship_icon_size,
                12.0,
                Some(textures.content_bounds()), // Crop the ship art's transparent padding, so it fills its icon/panel
            )
        };

        Self {
            main_banner: Banner::new(banner_kind, banner_pos, banner_size),
            vanguard_button: IconLabelButton::new(
                Rect::new(button_x, vanguard_button_y, button_size.x, button_size.y),
                ship_icon_for(ShipKind::Vanguard),
                ShipKind::Vanguard.display_name().to_string(),
                ShipKind::Vanguard.role().to_string(),
                button_title_font_size,
                button_subtitle_font_size,
            ),
            sentinel_button: IconLabelButton::new(
                Rect::new(button_x, sentinel_button_y, button_size.x, button_size.y),
                ship_icon_for(ShipKind::Sentinel),
                ShipKind::Sentinel.display_name().to_string(),
                ShipKind::Sentinel.role().to_string(),
                button_title_font_size,
                button_subtitle_font_size,
            ),
            viper_button: IconLabelButton::new(
                Rect::new(button_x, viper_button_y, button_size.x, button_size.y),
                ship_icon_for(ShipKind::Viper),
                ShipKind::Viper.display_name().to_string(),
                ShipKind::Viper.role().to_string(),
                button_title_font_size,
                button_subtitle_font_size,
            ),
            back_button: Button::new(
                Rect::new(back_button_pos.x, back_button_pos.y, back_button_size.x, back_button_size.y),
                "Back".to_string(),
                20,
                button_kind,
            ),
            preview_card: PreviewCard::new(
                Rect::new(preview_card_pos.x, preview_card_pos.y, preview_card_size.x, preview_card_size.y),
                ship_icon_for(default_ship_kind),
                default_ship_kind.display_name().to_string(),
                default_ship_kind.role().to_string(),
                ShipStats::preview_stats_for(default_ship_kind),
                "SELECT SHIP".to_string(),
            ),
            selected_ship_kind: default_ship_kind,
        }
    }
}

impl Drawable for ShipSelectionPage {
    fn draw(&self) {
        self.main_banner.draw();
        self.vanguard_button.draw();
        self.sentinel_button.draw();
        self.viper_button.draw();
        self.back_button.draw();
        self.preview_card.draw();
    }
}

impl StateUpdatable<()> for ShipSelectionPage {
    fn update_state(&mut self, _args: ()) {
        self.vanguard_button.update_state(());
        self.sentinel_button.update_state(());
        self.viper_button.update_state(());
        self.back_button.update_state(());
        self.preview_card.update_state(());
    }
}

impl Interactive for ShipSelectionPage {
    type Event = Option<ShipSelectionPageEvent>;

    /// Surfaces Events which would be helpful for external listeners.
    fn poll_event(&self) -> Self::Event {
        if let Some(ButtonEvents::Clicked) = self.back_button.poll_event() {
            return Some(ShipSelectionPageEvent::BackButtonPressed);
        }
        if let Some(PreviewCardEvent::ActionButtonClicked) = self.preview_card.poll_event() {
            return Some(ShipSelectionPageEvent::ShipConfirmed(self.selected_ship_kind));
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
        self.back_button
            .handle_self_event(self.back_button.poll_event());
        self.preview_card
            .handle_self_event(self.preview_card.poll_event());

        // A ship-button click pins that ship as the preview subject. The page
        // knows which button fired, so selection lives here, not on the buttons.
        let selected_ship = if matches!(self.vanguard_button.poll_event(), Some(ButtonEvents::Clicked)) {
            Some(ShipKind::Vanguard)
        } else if matches!(self.sentinel_button.poll_event(), Some(ButtonEvents::Clicked)) {
            Some(ShipKind::Sentinel)
        } else if matches!(self.viper_button.poll_event(), Some(ButtonEvents::Clicked)) {
            Some(ShipKind::Viper)
        } else {
            None
        };

        if let Some(ship_kind) = selected_ship {
            self.selected_ship_kind = ship_kind;

            let ship_sprites = &SpriteRepository::get_instance().ship_v1_sprite;
            let textures = ship_kind.ship_v1_texture_kind();
            self.preview_card.set_content(
                Animation::new(
                    ship_sprites.get_textures_for(&textures),
                    Vec2::new(140.0, 178.0),
                    12.0,
                    Some(textures.content_bounds()),
                ),
                ship_kind.display_name().to_string(),
                ship_kind.role().to_string(),
                ShipStats::preview_stats_for(ship_kind),
            );
        }
    }
}
