use macroquad::prelude::*;

use crate::game::{
    BASE_HEIGHT, BASE_WIDTH,
    asset_repository::{
        sprite_repository::{traits::{SpriteTextures, SpriteBounds}, SpriteRepository, ShipV1Textures},
        traits::Singleton,
    },
    entities::{animation::Animation, ship::ShipKind},
    interaction::{Interactive, SelfEventHandler},
    rendering::{Drawable, StateUpdatable},
    ui::components::{
        banner::{Banner, BannerKind},
        button::{Button, ButtonEvents, ButtonKind},
        icon_label_button::IconLabelButton,
        preview_card::{PreviewCard, PreviewCardEvent},
    },
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
    /// The ship currently highlighted and shown in the preview card.
    selected_ship_kind: ShipKind,
}

impl Default for ShipSelectionPage {
    fn default() -> Self {
        Self::new()
    }
}

impl ShipSelectionPage {
    pub fn new() -> Self {
        let x = 24.0;

        // Title banner in the top-left. Size off its height via the art's own
        // aspect ratio so it isn't stretched.
        let banner_kind = BannerKind::ShipSelectionTitle;
        let banner_height = BASE_HEIGHT * 0.18;
        let banner_width = banner_height * banner_kind.aspect_ratio();

        // --- Left column: the three ship buttons, stacked. ---
        let button_width = BASE_WIDTH * 0.30;
        // Derive height from the button art's own width:height so it isn't stretched.
        let button_height = button_width / ButtonKind::Basic.aspect_ratio();
        // Button trims the texture's transparent margins, so a button's bounds
        // are exactly its visible button art — this is a true gap between them.
        let gap = 14.0;
        // Center the column horizontally under the banner so the buttons sit
        // directly below the "SELECT SHIP" title.
        let button_x = x + (banner_width - button_width) / 2.0;
        // Start the column just below the banner rather than mid-page.
        let vanguard_y = 12.0 + banner_height + gap;
        let sentinel_y = vanguard_y + button_height + gap;
        let viper_y = sentinel_y + button_height + gap;
        let icon_size = Vec2::splat(button_height * 0.7);

        // --- Right: the preview card, centered in the space beside the buttons. ---
        let card_width = 320.0;
        let card_height = 410.0;
        let buttons_right = button_x + button_width;
        // Centered in the space beside the buttons, then biased a little right
        // so it isn't crowding the button column.
        let card_x = buttons_right + ((BASE_WIDTH - buttons_right) - card_width) / 2.0 + 40.0;
        let card_y = 100.0;

        // Small back button in the top-right corner — the banner already
        // occupies the top-left. Height derives from the art's aspect ratio
        // so the small size isn't stretched.
        let back_width = 90.0;
        let back_height = back_width / ButtonKind::Basic.aspect_ratio();

        // Only Sentinel has art today; reuse it as a placeholder for every ship.
        let ship_sprites = &SpriteRepository::get_instance().ship_v1_sprite;
        // Crop the ship art's transparent padding so it fills its icon/panel.
        let ship_crop = Some(ShipV1Textures::SentinelAlive.content_bounds());

        // The preview opens on the first ship so the card is never empty.
        let default_ship_kind = ShipKind::Vanguard;

        Self {
            main_banner: Banner::new(
                banner_kind,
                Vec2::new(x, 12.0),
                Vec2::new(banner_width, banner_height),
            ),
            vanguard_button: IconLabelButton::new(
                Rect::new(button_x, vanguard_y, button_width, button_height),
                Animation::new(
                        ship_sprites.get_textures_for(&ShipV1Textures::SentinelAlive),
                        icon_size,
                        12.0,
                        ship_crop,
                    ),
                ShipKind::Vanguard.display_name().to_string(),
                ShipKind::Vanguard.role().to_string(),
            ),
            sentinel_button: IconLabelButton::new(
                Rect::new(button_x, sentinel_y, button_width, button_height),
                Animation::new(
                        ship_sprites.get_textures_for(&ShipV1Textures::SentinelAlive),
                        icon_size,
                        12.0,
                        ship_crop,
                    ),
                ShipKind::Sentinel.display_name().to_string(),
                ShipKind::Sentinel.role().to_string(),
            ),
            viper_button: IconLabelButton::new(
                Rect::new(button_x, viper_y, button_width, button_height),
                Animation::new(
                        ship_sprites.get_textures_for(&ShipV1Textures::SentinelAlive),
                        icon_size,
                        12.0,
                        ship_crop,
                    ),
                ShipKind::Viper.display_name().to_string(),
                ShipKind::Viper.role().to_string(),
            ),
            back_button: Button::new(
                Rect::new(BASE_WIDTH - back_width - 20.0, 15.0, back_width, back_height),
                "Back".to_string(),
                20,
                ButtonKind::Basic,
            ),
            preview_card: PreviewCard::new(
                Rect::new(card_x, card_y, card_width, card_height),
                Animation::new(
                        ship_sprites.get_textures_for(&ShipV1Textures::SentinelAlive),
                        icon_size,
                        12.0,
                        ship_crop,
                    ),
                default_ship_kind.display_name().to_string(),
                default_ship_kind.role().to_string(),
                {
                    let stats = default_ship_kind.stats();
                    vec![
                        ("Damage".to_string(), format!("{}", stats.fire_damage() as i32)),
                        ("Defense".to_string(), format!("{}", stats.max_health() as i32)),
                        ("Speed".to_string(), format!("{}", stats.speed() as i32)),
                        ("Guns".to_string(), format!("{}", stats.gun_count())),
                    ]
                },
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

        // A ship-button click pins that ship as the preview subject. The page
        // knows which button fired, so selection lives here, not on the buttons.
        let clicked = if matches!(self.vanguard_button.poll_event(), Some(ButtonEvents::Clicked)) {
            Some(ShipKind::Vanguard)
        } else if matches!(self.sentinel_button.poll_event(), Some(ButtonEvents::Clicked)) {
            Some(ShipKind::Sentinel)
        } else if matches!(self.viper_button.poll_event(), Some(ButtonEvents::Clicked)) {
            Some(ShipKind::Viper)
        } else {
            None
        };
        if let Some(ship_kind) = clicked {
            self.selected_ship_kind = ship_kind;

            // Only Sentinel has art today; reuse it as a placeholder for every ship.
            let ship_sprites = &SpriteRepository::get_instance().ship_v1_sprite;
            let ship_crop = Some(ShipV1Textures::SentinelAlive.content_bounds());
            let stats = ship_kind.stats();
            self.preview_card.set_content(
                Animation::new(
                    ship_sprites.get_textures_for(&ShipV1Textures::SentinelAlive),
                    Vec2::new(140.0, 178.0),
                    12.0,
                    ship_crop,
                ),
                ship_kind.display_name().to_string(),
                ship_kind.role().to_string(),
                vec![
                    ("Damage".to_string(), format!("{}", stats.fire_damage() as i32)),
                    ("Defense".to_string(), format!("{}", stats.max_health() as i32)),
                    ("Speed".to_string(), format!("{}", stats.speed() as i32)),
                    ("Guns".to_string(), format!("{}", stats.gun_count())),
                ],
            );
        }
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
    }
}