use macroquad::prelude::*;

use crate::game::{
    entities::animation::Animation,
    interaction::{Interactive, SelfEventHandler},
    object::HasId,
    rendering::{Drawable, StateUpdatable},
    ui::components::button::{Button, ButtonEvents},
};

/// A `Button` with an icon and a two-line title/subtitle label instead of
/// `Button`'s single centered line — e.g. a "FALCON — Balanced Fighter"
/// entry in a ship-select list. Wraps `Button` for its background art and
/// click/hover behavior (`draw`/`update_state`/`poll_event`/
/// `handle_self_event` all delegate to it), layering the icon and text on
/// top instead of `Button`'s own label draw.
pub struct IconLabelButton {
    button: Button,
    icon: Animation,
    title: String,
    subtitle: String,
}

impl IconLabelButton {
    /// `icon` is drawn on the left; `title`/`subtitle` are drawn as two
    /// left-aligned lines to its right.
    pub fn new(bounds: Rect, icon: Animation, title: String, subtitle: String) -> Self {
        Self {
            button: Button::new(bounds, String::new(), 16),
            icon,
            title,
            subtitle,
        }
    }

    /// This button's on-screen position and size, so an owning page can draw
    /// a selection highlight around it.
    pub fn bounds(&self) -> Rect {
        self.button.bounds()
    }
}

impl StateUpdatable<()> for IconLabelButton {
    fn update_state(&mut self, args: ()) {
        self.button.update_state(args);
    }
}

impl Drawable for IconLabelButton {
    fn draw(&self) {
        self.button.draw();

        let bounds = self.button.bounds();

        // Keeps content off the background art's beveled border (inner
        // ~72%/50% is the actual blue panel).
        let inset_x = bounds.w * 0.14;
        let inset_y = bounds.h * 0.25;

        let icon_size = *self.icon.frame_scale();
        let icon_x = bounds.x + inset_x;
        let icon_y = bounds.y + (bounds.h - icon_size.y) / 2.0;
        draw_texture_ex(
            self.icon.current_frame(),
            icon_x,
            icon_y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(icon_size),
                ..Default::default()
            },
        );

        let text_x = icon_x + icon_size.x + inset_x * 0.6;
        let _ = inset_y;
        let title_font_size = 24;
        let subtitle_font_size = 16;
        // Warm gold instead of a blue tone — a blue subtitle barely reads
        // against this button's own blue background art.
        let subtitle_color = Color::new(1.0, 0.85, 0.4, 1.0);
        let title_ts = measure_text(&self.title, None, title_font_size, 1.0);
        draw_text_ex(
            &self.title,
            text_x,
            bounds.y + bounds.h / 2.0 - 4.0,
            TextParams {
                font_size: title_font_size,
                color: WHITE,
                ..Default::default()
            },
        );
        draw_text_ex(
            &self.subtitle,
            text_x,
            bounds.y + bounds.h / 2.0 + title_ts.offset_y,
            TextParams {
                font_size: subtitle_font_size,
                color: subtitle_color,
                ..Default::default()
            },
        );
    }
}

impl HasId for IconLabelButton {
    fn id(&self) -> u64 {
        self.button.id()
    }
}

impl Interactive for IconLabelButton {
    type Event = Option<ButtonEvents>;
    fn poll_event(&self) -> Self::Event {
        self.button.poll_event()
    }
}

impl SelfEventHandler for IconLabelButton {
    fn handle_self_event(&mut self, event: Self::Event) {
        self.button.handle_self_event(event);
    }
}
