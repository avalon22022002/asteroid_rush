use macroquad::prelude::*;

use crate::game::{BASE_WIDTH, traits::rendering::{Drawable, StateUpdatable}};

pub struct HudData {
    pub score: u32,
    pub cur_health: u32,
    pub max_health: u32,
}

/// The gameplay HUD (heads-up display): the always-on overlay showing the
/// player hull health top-left and score/time centered at top. 
/// Purely a display: just renders whatever `HudData` it was last given.
pub struct Hud {
    data: HudData,
}

impl Hud {
    pub fn new() -> Self {
        Self { data: HudData { score: 0, cur_health: 0, max_health: 0 } }
    }

    /// Draws `text` with a dark drop-shadow so it stays legible over the starfield.
    fn draw_text_legible(text: &str, x: f32, y: f32, font_size: u16, color: Color) {
        const SHADOW: Color = Color::new(0.0, 0.0, 0.0, 0.85);
        draw_text_ex(text, x + 1.0, y + 1.0, TextParams { font_size, color: SHADOW, ..Default::default() });
        draw_text_ex(text, x, y, TextParams { font_size, color, ..Default::default() });
    }

    fn draw_health_panel(&self) {
        const PAD: f32 = 20.0;
        const BAR_PAD: f32 = 36.0;
        const BAR_BACKGROUND: Color = Color::new(0.0, 0.0, 0.0, 0.35);
        const HEALTHY: Color = Color::new(0.3, 0.85, 0.5, 1.0);
        const WARNING: Color = Color::new(1.0, 0.85, 0.4, 1.0);
        const CRITICAL: Color = Color::new(1.0, 0.36, 0.36, 1.0);

        let bounds = Rect::new(20.0, 15.0, 230.0, 62.0);
        let (cur_health, max_health) = (self.data.cur_health, self.data.max_health);
        let text_y = bounds.y + 25.0;

        Self::draw_text_legible("Ship Health", bounds.x + PAD, text_y, 15, WHITE);

        let value = format!("{cur_health} / {max_health}");
        let value_width = measure_text(&value, None, 15, 1.0).width;
        Self::draw_text_legible(&value, bounds.right() - PAD - value_width, text_y, 15, WHITE);

        let ratio = if max_health > 0 { (cur_health as f32 / max_health as f32).clamp(0.0, 1.0) } else { 0.0 };
        let bar_color = if ratio > 0.5 { HEALTHY } else if ratio > 0.25 { WARNING } else { CRITICAL };

        let bar = Rect::new(bounds.x + BAR_PAD, bounds.y + 35.0, bounds.w - BAR_PAD * 2.0, 10.0);
        draw_rectangle(bar.x, bar.y, bar.w, bar.h, BAR_BACKGROUND);
        draw_rectangle(bar.x, bar.y, bar.w * ratio, bar.h, bar_color);
    }

    fn draw_center_panel(&self) {
        const DIVIDER_COLOR: Color = Color::new(0.4, 0.7, 1.0, 0.5);
        const SCORE_COLOR: Color = Color::new(1.0, 0.85, 0.4, 1.0);

        let bounds = Rect::new((BASE_WIDTH - 260.0) / 2.0, 15.0, 260.0, 62.0);

        let mid_x = bounds.x + bounds.w / 2.0;
        draw_line(mid_x, bounds.y + 16.0, mid_x, bounds.bottom() - 10.0, 1.0, DIVIDER_COLOR);

        let top = bounds.y + 22.0;
        let score_text = self.data.score.to_string();
        Self::draw_stat(bounds.x + bounds.w / 4.0, top, "SCORE", &score_text, SCORE_COLOR);

        // Time-remaining isn't wired up yet.
        Self::draw_stat(bounds.x + bounds.w * 3.0 / 4.0, top, "TIME", "--:--", WHITE);
    }

    /// Draws one stat column: `label` above `value`, both centered on `center_x`.
    fn draw_stat(center_x: f32, top_y: f32, label: &str, value: &str, value_color: Color) {
        let label_width = measure_text(label, None, 14, 1.0).width;
        Self::draw_text_legible(label, center_x - label_width / 2.0, top_y, 14, WHITE);

        let value_width = measure_text(value, None, 24, 1.0).width;
        Self::draw_text_legible(value, center_x - value_width / 2.0, top_y + 21.0, 24, value_color);
    }
}

impl Drawable for Hud {
    fn draw(&self) {
        self.draw_health_panel();
        self.draw_center_panel();
    }
}

impl StateUpdatable<HudData> for Hud {
    fn update_state(&mut self, data: HudData) {
        self.data = data;
    }
}
