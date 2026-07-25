use macroquad::{prelude::*};

use crate::game::{BASE_WIDTH, interaction::{Interactive,EventHandler}, utils, object::HasId, rendering::{Drawable, StateUpdatable}};

pub struct Button {
    id: u64,
    bounds: Rect,
    label: String,
    color: Color,
    font_size: u16,
    accent_color: Option<Color>,
    hovered: bool,
    clicked: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Events{
    Clicked,
}

impl Button {
    /// `accent_color` sets the color of the button's bottom accent bar, e.g.
    /// green to mark a confirming action or red for a destructive one. Pass
    /// `None` to fall back to a subtle dark bezel shine.
   pub fn new(bounds: Rect, label: String, color: Color, font_size:  u16, accent_color: Option<Color>)-> Self{
        Self { id: utils::get_next_unique_id(), bounds, label, color, font_size, accent_color, hovered: false, clicked: false }
    }

    /// Returns `true` if the mouse cursor is currently within the button's bounds.
    fn is_mouse_over(&self) -> bool {
        let (mx, my) = mouse_position();
        self.bounds.contains(Vec2::new(mx, my))
    }

    /// Returns `true` on the exact frame the button is clicked via the mouse —
    /// i.e. the cursor is over the button and the left mouse button was just
    /// pressed this frame. Edge-triggered (`is_mouse_button_pressed`), so this
    /// fires once per click rather than every frame the button is held down.
    fn is_clicked_via_mouse(&self) -> bool {
        self.is_mouse_over() && is_mouse_button_pressed(MouseButton::Left)
    }

    fn play_click_sound(){
        
    }
}

impl StateUpdatable<()> for Button {
    /// Refreshes `hovered`/`clicked` from the current mouse state. This only
    /// updates the button's own state for rendering — it does not decide what
    /// a click *means*. Consumers read `poll_event` (via `Interactive`) after
    /// this to react to the click, keeping Button decoupled from whatever
    /// action it triggers.
    fn update_state(&mut self, _args: ()) {
        self.hovered = self.is_mouse_over();
        self.clicked = self.is_clicked_via_mouse();
    }
}

impl Drawable for Button {
    fn draw(&self) {
        let scale = screen_width() / BASE_WIDTH;
        let border_thickness = 2.0 * scale;
        let accent_height = 4.0 * scale;

        let max_font_from_height = (self.bounds.h * 0.55) as u16;
        let mut font_size = (self.font_size.min(max_font_from_height) as f32 * scale).round() as u16;
        font_size = font_size.max(8);

        loop {
            let ts = measure_text(&self.label, None, font_size, 1.0);
            if ts.width <= self.bounds.w - 8.0 * scale || font_size <= 8 {
                break;
            }
            font_size -= 1;
        }

        let shadow_offset = 3.0 * scale;
        draw_rectangle(
            self.bounds.x + shadow_offset,
            self.bounds.y + shadow_offset,
            self.bounds.w,
            self.bounds.h,
            Color::new(0.0, 0.0, 0.0, 0.35),
        );

        let base_color = if self.clicked {
            Color::new(
                (self.color.r - 0.15).max(0.0),
                (self.color.g - 0.15).max(0.0),
                (self.color.b - 0.15).max(0.0),
                self.color.a,
            )
        } else if self.hovered {
            Color::new(
                (self.color.r + 0.15).min(1.0),
                (self.color.g + 0.15).min(1.0),
                (self.color.b + 0.15).min(1.0),
                self.color.a,
            )
        } else {
            self.color
        };
        draw_rectangle(self.bounds.x, self.bounds.y, self.bounds.w, self.bounds.h, base_color);

        draw_rectangle(
            self.bounds.x,
            self.bounds.y,
            self.bounds.w,
            accent_height,
            Color::new(1.0, 1.0, 1.0, 0.25),
        );

        draw_rectangle(
            self.bounds.x,
            self.bounds.y + self.bounds.h - accent_height,
            self.bounds.w,
            accent_height,
            self.accent_color.unwrap_or(Color::new(0.0, 0.0, 0.0, 0.25)),
        );

        draw_rectangle_lines(
            self.bounds.x,
            self.bounds.y,
            self.bounds.w,
            self.bounds.h,
            border_thickness,
            WHITE,
        );

        let ts = measure_text(&self.label, None, font_size, 1.0);
        draw_text_ex(
            &self.label,
            self.bounds.x + (self.bounds.w - ts.width) / 2.0,
            self.bounds.y + self.bounds.h / 2.0 + ts.offset_y / 2.0,
            TextParams {
                font_size,
                color: WHITE,
                ..Default::default()
            },
        );
    }
}

impl HasId for Button {
    fn id(&self) -> u64 {
        self.id
    }
}

impl Interactive for Button{
    type Event = Option<Events>;
    fn poll_event(&self) -> Self::Event {
        if self.clicked {
            return Some(Events::Clicked)
        }
        return None;
    }
}

impl EventHandler for Button{
    fn handle_event(&mut self, event: Self::Event) {
        
    }
}