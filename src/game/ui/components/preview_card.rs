use macroquad::prelude::*;

use crate::game::{
    entities::animation::Animation, interaction::{Interactive, SelfEventHandler}, rendering::{Drawable, StateUpdatable}, ui::components::{button::{Button, ButtonEvents, ButtonKind}},
};

/// Height of one stat row in the scrollable list, in logical pixels.
const ROW_HEIGHT: f32 = 24.0;
/// Logical pixels one wheel notch scrolls the stat list.
const SCROLL_SPEED: f32 = 24.0;
/// Scroll Bar Width
const SCROLLBAR_WIDTH: f32 = 4.0;

/// A vertical card previewing one item: art panel, title + subtitle, a
/// scrollable list of `label: value` rows, and an action button. Ignorant of
/// what it shows — callers pass plain strings and an `Animation` via
/// `set_content`, so it can back a ship, level, or weapon preview alike.
pub struct PreviewCard {
    bounds: Rect,
    art: Animation,
    title: String,
    subtitle: String,
    /// `(label, value)` pairs, drawn top-to-bottom in the stats viewport.
    label_value_pairs: Vec<(String, String)>,
    action_button: Button,
    layout: CardLayout,
    /// Stat-list scroll position in pixels.
    scroll_offset: f32,
}

/// The card's sub-regions, derived from previewCard's `bounds`
struct CardLayout {
    animation_panel_bounds: Rect,
    title_baseline_y: f32,
    subtitle_baseline_y: f32,
    stats_viewport: Rect,
    button: Rect,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreviewCardEvent {
    /// The action button was clicked, confirming this card's item.
    ActionButtonClicked,
}

impl PreviewCard {
    fn compute_card_layout(card_bounds: Rect) -> CardLayout {
        let pad = 10.0;
        let button = Rect::new(
            card_bounds.x + pad,
            card_bounds.bottom() - pad - 38.0,
            card_bounds.w - 2.0 * pad,
            38.0,
        );

        // Title + subtitle sit just above the button, at the bottom of the card.
        let subtitle_baseline_y = button.top() - 8.0;
        let title_baseline_y = subtitle_baseline_y - 22.0;

        // Art gets the top half of the card; the rest leaves room for the full
        // stat list, title, subtitle, and button without any of them scrolling.
        // Inset the top by `pad` so the art doesn't touch the card's border.
        let animation_panel_bounds = Rect::new(
            card_bounds.x,
            card_bounds.y + pad,
            card_bounds.w,
            card_bounds.h * 0.5 - pad,
        );

        // Stats fill the gap between the art and the title, scrolling if they
        // don't all fit.
        let stats_top = animation_panel_bounds.bottom() + 5.0;
        let stats_bottom = title_baseline_y - 16.0;
        let stats_viewport = Rect::new(
            card_bounds.x + pad,
            stats_top,
            card_bounds.w - 2.0 * pad,
            (stats_bottom - stats_top).max(0.0),
        );

        CardLayout {
            animation_panel_bounds,
            title_baseline_y,
            subtitle_baseline_y,
            stats_viewport,
            button,
        }
    }

    pub fn new(
        bounds: Rect,
        art: Animation,
        title: String,
        subtitle: String,
        label_value_pairs: Vec<(String, String)>,
        action_label: String,
    ) -> Self {
        let layout = PreviewCard::compute_card_layout(bounds);
        Self {
            bounds,
            art,
            title,
            subtitle,
            label_value_pairs,
            action_button: Button::new(layout.button, action_label, 24, ButtonKind::Basic),
            layout,
            scroll_offset: 0.0
        }
    }

    /// Swaps what the card shows, keeping its bounds and action button.
    pub fn set_content(
        &mut self,
        art: Animation,
        title: String,
        subtitle: String,
        label_value_pairs: Vec<(String, String)>
    ) {
        self.art = art;
        self.title = title;
        self.subtitle = subtitle;
        self.label_value_pairs = label_value_pairs;
    }

    /// Total pixel height of all stat rows.
    fn content_height(&self) -> f32 {
        self.label_value_pairs.len() as f32 * ROW_HEIGHT
    }

    /// Draws the preview card's border: a semi transparent dark fill with a thin blue outline.
    fn draw_preview_card_border(&self) {
        // Draw semi transparent Dark Fill in Card
        draw_rectangle(
            self.bounds.x,
            self.bounds.y,
            self.bounds.w,
            self.bounds.h,
            Color::new(0.05, 0.08, 0.15, 0.55),
        );

        // Draw Card border with thin blue lines
        draw_rectangle_lines(
            self.bounds.x,
            self.bounds.y,
            self.bounds.w,
            self.bounds.h,
            2.0,
            Color::new(0.4, 0.7, 1.0, 0.6),
        );
    }

    /// The art centered in `panel` at its own aspect ratio so it isn't
    /// distorted. Drawn straight over the page background — no panel behind it.
    fn draw_animation_in_card(&self) {
        let frame = self.art.current_frame();
        let animation_panel = self.layout.animation_panel_bounds;

        // Shrink the panel slightly on the width so the art never touches the edges.
        let padded_panel = Rect::new(
            animation_panel.x,
            animation_panel.y,
            animation_panel.w * 0.95,
            animation_panel.h,
        );
        // Fit by the art's drawn content (crop, if any) so transparent padding
        // doesn't skew the scale or leave the art floating small in the panel.
        let content = self.art.content_size();
        let scale = (padded_panel.h / content.y).min(padded_panel.w / content.x);
        let art_size = content * scale;
        let frame_rect = Rect::new(
            padded_panel.x + (padded_panel.w - art_size.x) / 2.0,
            padded_panel.y + (padded_panel.h - art_size.y) / 2.0,
            art_size.x,
            art_size.y,
        );

        draw_texture_ex(
            frame,
            frame_rect.x,
            frame_rect.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(frame_rect.size()),
                source: self.art.frame_crop(),
                ..Default::default()
            },
        );
    }

    /// The `label: value` rows, offset by `scroll_offset`, with a scrollbar
    /// when they overflow. Rows spilling outside `viewport` are skipped so the
    /// list can't paint over the art or the title.
    fn draw_stats_in_card(&self) {
        self.draw_stat_rows_in_card();
        self.draw_scrollbar_in_card();
    }

    /// Draws each visible row: label left-aligned, value right-aligned next to the scrollbar.
    fn draw_stat_rows_in_card(&self) {
        let viewport = self.layout.stats_viewport;
        const LABEL_SIZE: u16 = 18;
        const VALUE_COLOR: Color = Color::new(0.4, 0.7, 1.0, 1.0);

        // True if a row starting at `row_top` falls outside `viewport`, with
        // 0.5px slack to absorb rounding at the edges.
        let row_outside_viewport = |row_top: f32| -> bool {
            row_top < viewport.y - 0.5 || row_top + ROW_HEIGHT > viewport.bottom() + 0.5
        };

        for (i, (label, value)) in self.label_value_pairs.iter().enumerate() {
            let row_top = viewport.y + i as f32 * ROW_HEIGHT - self.scroll_offset;
            if row_outside_viewport(row_top) {
                continue;
            }
            let baseline_y = row_top + ROW_HEIGHT * 0.7;
            
            // draw label(key) from label_value_pairs in self.label_value_pairs
            draw_text_ex(
                label,
                viewport.x,
                baseline_y,
                TextParams { font_size: LABEL_SIZE, color: WHITE, ..Default::default() },
            );

            // draw value from label_value_pairs in self.label_value_pairs
            let value_width = measure_text(value, None, LABEL_SIZE, 1.0).width;
            let value_pos_x = viewport.right() - SCROLLBAR_WIDTH - 6.0 - value_width;
            draw_text_ex(
                value,
                value_pos_x,
                baseline_y,
                TextParams { font_size: LABEL_SIZE, color: VALUE_COLOR, ..Default::default() },
            );
        }
    }

    /// Draws the scrollbar for the stats list, only while rows overflow the
    /// viewport. Two pieces: `track` is the full-length background groove;
    /// `handle` is the smaller draggable-looking bar inside it that shows how
    /// much content is visible and where you currently are within it.
    fn draw_scrollbar_in_card(&self) {
        let viewport = self.layout.stats_viewport;
        let content_h = self.content_height();

        // Nothing to scroll — don't draw a scrollbar at all.
        if content_h <= viewport.h {
            return;
        }

        // Track: dim strip spanning the full viewport height, right-aligned.
        let track_x = viewport.right() - SCROLLBAR_WIDTH;
        draw_rectangle(track_x, viewport.y, SCROLLBAR_WIDTH, viewport.h, Color::new(1.0, 1.0, 1.0, 0.15));

        // Handle height = the fraction of content currently visible (viewport / content),
        // scaled to pixels. Floored at 14px so it's never too small to see/click.
        let handle_h = (viewport.h * viewport.h / content_h).max(14.0);

        // Scroll progress as a 0.0-1.0 fraction: 0 = top of content, 1 = bottom.
        let max_offset = content_h - viewport.h;
        let t = if max_offset > 0.0 { self.scroll_offset / max_offset } else { 0.0 };

        // Handle's own height eats into its travel range, so its position is
        // t applied to (track height - handle height), not the full track.
        let handle_y = viewport.y + t * (viewport.h - handle_h);

        // Handle: brighter and more opaque than the track so it stands out.
        draw_rectangle(track_x, handle_y, SCROLLBAR_WIDTH, handle_h, Color::new(0.4, 0.7, 1.0, 0.85));
    }

    /// Draws `text` horizontally centered in the card at `baseline_y`.
    fn draw_text_centered(&self, text: &str, baseline_y: f32, font_size: u16, color: Color) {
        let ts = measure_text(text, None, font_size, 1.0);
        draw_text_ex(
            text,
            self.bounds.x + (self.bounds.w - ts.width) / 2.0,
            baseline_y,
            TextParams {
                font_size,
                color,
                ..Default::default()
            },
        );
    }
}

impl Drawable for PreviewCard {
    fn draw(&self) {
        // Draw Card Border
        self.draw_preview_card_border();
        // Draw Card Animation
        self.draw_animation_in_card();
        // Draw Card Title
        self.draw_text_centered(
            &self.title,
            self.layout.title_baseline_y,
            30,
            Color::new(0.4, 0.7, 1.0, 1.0),
        );
        // Draw Card Subtitle
        self.draw_text_centered(
            &self.subtitle,
            self.layout.subtitle_baseline_y,
            18,
            Color::new(1.0, 0.85, 0.4, 1.0),
        );
        // Draw Card Stats
        self.draw_stats_in_card();
        // Draw Card Action button
        self.action_button.draw();
    }
}

impl StateUpdatable<()> for PreviewCard {
    fn update_state(&mut self, _args: ()) {
        self.art.advance(get_frame_time());
        self.action_button.update_state(());

        // Scroll only while the cursor is over the stat viewport, so the
        // wheel doesn't steal scrolling meant for the rest of the page.
        let (mx, my) = mouse_position();
        if self.layout.stats_viewport.contains(Vec2::new(mx, my)) {
            let (_, wheel_y) = mouse_wheel();
            if wheel_y != 0.0 {
                // Wheel-down (negative) reveals lower rows, i.e. more offset.
                self.scroll_offset -= wheel_y.signum() * SCROLL_SPEED;
                let max = (self.content_height() - self.layout.stats_viewport.h).max(0.0);
                self.scroll_offset = self.scroll_offset.clamp(0.0, max);
            }
        }
    }
}

impl Interactive for PreviewCard {
    type Event = Option<PreviewCardEvent>;
    fn poll_event(&self) -> Self::Event {
        if let Some(ButtonEvents::Clicked) = self.action_button.poll_event() {
            return Some(PreviewCardEvent::ActionButtonClicked);
        }
        None
    }
}

impl SelfEventHandler for PreviewCard {
    fn handle_self_event(&mut self, _event: Self::Event) {
        self.action_button
            .handle_self_event(self.action_button.poll_event());
    }
}
