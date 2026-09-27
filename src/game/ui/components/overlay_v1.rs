use macroquad::prelude::*;

use crate::game::{
    BASE_HEIGHT, BASE_WIDTH,
    asset_repository::sprite_repository::traits::SpriteBounds,
    traits::interaction::{Interactive, SelfEventHandler},
    traits::rendering::{Drawable, StateUpdatable},
    ui::components::button::{Button, ButtonEvents, ButtonKind},
    utils::aspect_size_from_fixed_width,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OverlayV1Event {
    Option1Clicked,
    Option2Clicked,
}

/// The popup's on-screen positions, computed once so `draw` just reads fixed
/// numbers instead of re-deriving them every frame.
struct OverlayV1Layout {
    panel_bounds: Rect,
    title_position: Vec2,
    title_font_size: u16,
    subtitle_position: Option<Vec2>,
    subtitle_font_size: u16,
    option_1_button_bounds: Rect,
    option_2_button_bounds: Rect,
}

/// A generic popup that covers the whole screen: a dim backdrop behind a
/// centered panel with a title and two choices. Carries no meaning of its
/// own — callers give it a title and two option labels and interpret which
/// option fired, so it can back a pause menu, a game-over screen, a confirm
/// dialog, etc.
pub struct OverlayV1 {
    title_label: String,
    subtitle_label: Option<String>,
    option_1_button: Button,
    option_2_button: Button,
    layout: OverlayV1Layout,
}

impl OverlayV1 {
    /// Sizes the panel to fit its own content — a title, an optional
    /// subtitle, and two stacked buttons — instead of a hardcoded height the
    /// buttons could outgrow, centers it on screen, and derives every
    /// sub-element's position from that panel.
    fn calculate_layout(title: &str, subtitle: Option<&str>) -> OverlayV1Layout {
        // Panel config
        let panel_width = 360.0;
        let subtitle_font_size = 20;
        // Space above the buttons, reserved for the title (and subtitle, if any).
        let top_panel_padding = if subtitle.is_some() { 110.0 } else { 80.0 };
        let bottom_panel_padding = 24.0;

        // Option button common config
        let option_button_kind = ButtonKind::Basic;
        let option_button_size = aspect_size_from_fixed_width(panel_width - 60.0, option_button_kind.aspect_ratio());
        let option_button_vertical_gap = 14.0; // Vertical spacing between the two buttons

        // Panel height is derived from its content: title space, two buttons,
        // the gap between them, and the top and bottom padding.
        let panel_height = top_panel_padding + option_button_size.y * 2.0 + option_button_vertical_gap + bottom_panel_padding;
        let panel_size = Vec2::new(panel_width, panel_height);
        let panel_bounds = Rect::new(
            (BASE_WIDTH - panel_size.x) / 2.0,
            (BASE_HEIGHT - panel_size.y) / 2.0,
            panel_size.x,
            panel_size.y,
        );

        // Adjust each button's position so they're centered horizontally and stacked with a vertical gap between them.
        let option_button_x = panel_bounds.x + (panel_size.x - option_button_size.x) / 2.0;
        let option_1_button_y = panel_bounds.y + top_panel_padding;
        let option_2_button_y = option_1_button_y + option_button_size.y + option_button_vertical_gap;

        // Title sits centered near the panel's top, at a fixed offset so
        // adding a subtitle doesn't shift it.
        let title_font_size = 32;
        let title_size = measure_text(title, None, title_font_size, 1.0);
        let title_position = Vec2::new(
            panel_bounds.x + (panel_size.x - title_size.width) / 2.0,
            panel_bounds.y + 40.0,
        );

        // Subtitle, if any, sits centered just below the title.
        let subtitle_position = subtitle.map(|subtitle| {
            let subtitle_size = measure_text(subtitle, None, subtitle_font_size, 1.0);
            Vec2::new(panel_bounds.x + (panel_size.x - subtitle_size.width) / 2.0, title_position.y + 34.0)
        });

        OverlayV1Layout {
            panel_bounds,
            title_position,
            title_font_size,
            subtitle_position,
            subtitle_font_size,
            option_1_button_bounds: Rect::new(option_button_x, option_1_button_y, option_button_size.x, option_button_size.y),
            option_2_button_bounds: Rect::new(option_button_x, option_2_button_y, option_button_size.x, option_button_size.y),
        }
    }

    pub fn new(title_label: String, subtitle_label: Option<String>, option_1_label: String, option_2_label: String) -> Self {
        let layout = Self::calculate_layout(&title_label, subtitle_label.as_deref());

        Self {
            title_label,
            subtitle_label,
            option_1_button: Button::new(layout.option_1_button_bounds, option_1_label, 28, ButtonKind::Basic),
            option_2_button: Button::new(layout.option_2_button_bounds, option_2_label, 28, ButtonKind::Basic),
            layout,
        }
    }

    /// Draws a black rectangle with 60% opacity over the entire screen,
    /// dimming the content behind the overlay.
    fn draw_overlay_background(&self) {
        draw_rectangle(0.0, 0.0, BASE_WIDTH, BASE_HEIGHT, Color::new(0.0, 0.0, 0.0, 0.6));
    }

    fn draw_panel(&self) {
        let bounds = self.layout.panel_bounds;
        draw_rectangle(bounds.x, bounds.y, bounds.w, bounds.h, Color::new(0.05, 0.08, 0.15, 0.9));
        draw_rectangle_lines(bounds.x, bounds.y, bounds.w, bounds.h, 2.0, Color::new(0.4, 0.7, 1.0, 0.6));
    }

    fn draw_title(&self) {
        draw_text_ex(
            &self.title_label,
            self.layout.title_position.x,
            self.layout.title_position.y,
            TextParams { font_size: self.layout.title_font_size, color: WHITE, ..Default::default() },
        );
    }

    fn draw_subtitle(&self) {
        let (Some(subtitle), Some(position)) = (&self.subtitle_label, self.layout.subtitle_position) else { return };
        draw_text_ex(
            subtitle,
            position.x,
            position.y,
            TextParams { font_size: self.layout.subtitle_font_size, color: Color::new(1.0, 0.85, 0.4, 1.0), ..Default::default() },
        );
    }
}

impl Drawable for OverlayV1 {
    fn draw(&self) {
        self.draw_overlay_background();
        self.draw_panel();
        self.draw_title();
        self.draw_subtitle();
        self.option_1_button.draw();
        self.option_2_button.draw();
    }
}

impl StateUpdatable<()> for OverlayV1 {
    fn update_state(&mut self, _args: ()) {
        self.option_1_button.update_state(());
        self.option_2_button.update_state(());
    }
}

impl Interactive for OverlayV1 {
    type Event = Option<OverlayV1Event>;
    fn poll_event(&self) -> Self::Event {
        if let Some(ButtonEvents::Clicked) = self.option_1_button.poll_event() {
            return Some(OverlayV1Event::Option1Clicked);
        }
        if let Some(ButtonEvents::Clicked) = self.option_2_button.poll_event() {
            return Some(OverlayV1Event::Option2Clicked);
        }
        None
    }
}

impl SelfEventHandler for OverlayV1 {
    fn handle_self_event(&mut self, _event: Self::Event) {
        self.option_1_button
            .handle_self_event(self.option_1_button.poll_event());
        self.option_2_button
            .handle_self_event(self.option_2_button.poll_event());
    }
}
