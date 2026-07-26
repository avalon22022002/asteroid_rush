use std::sync::OnceLock;

use macroquad::prelude::*;

use crate::game::{
    audio::{self, AudioName},
    interaction::{Interactive, SelfEventHandler},
    object::HasId,
    rendering::{Drawable, StateUpdatable},
    utils,
};

const LOG_PREFIX: &str = "[button]";

/// # Example
///
/// ```ignore
/// let mut new_game_button = Button::new(
///     Rect::new(20.0, 100.0, 200.0, 48.0),
///     "New Game".to_string(),
///     30,
///     None, // use the default button-background.png art
/// );
///
/// loop {
///     // Refresh hover/clicked state from this frame's input.
///     new_game_button.update_state(());
///     new_game_button.draw();
///
///     // Let the button react to its own event first — self-contained
///     // feedback it owns, like playing the click sound. `event` is
///     // `Copy`, so reading it here doesn't stop the consumer below from
///     // reading it too.
///     let event = new_game_button.poll_event();
///     new_game_button.handle_self_event(event);
///
///     // Then the consumer decides what the click *means* for the app.
///     // That's custom app logic (page navigation, game state, ...), which
///     // `SelfEventHandler` deliberately stays out of — the button has no
///     // notion of "pages".
///     if let Some(ButtonEvents::Clicked) = event {
///         println!("New Game clicked — switch to the game page");
///     }
///
///     next_frame().await;
/// }
/// ```
pub struct Button {
    id: u64,
    bounds: Rect,
    label: String,
    font_size: u16,
    texture: Texture2D,
    is_mouse_over: bool,
    was_hovered: bool,
    clicked: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonEvents {
    /// Fires once on the frame the button is clicked via the mouse.
    Clicked,
    /// Fires once on the frame the cursor enters the button's bounds.
    HoverStarted,
    /// Fires every frame the cursor remains within the button's bounds,
    /// after the entry frame (which reports `HoverStarted` instead). Use
    /// this for feedback that should track the cursor continuously rather
    /// than fire once, e.g. drawing a tooltip.
    Hovering,
    /// Fires once on the frame the cursor leaves the button's bounds.
    HoverEnded,
}

impl Button {
    /// `texture` is the background art drawn across `bounds`. Pass `None` to
    /// use the shared default (`assets/ui/button/button-background.png`), or
    /// `Some(..)` to give this button its own art.
    pub fn new(bounds: Rect, label: String, font_size: u16, texture: Option<Texture2D>) -> Self {
        Self {
            id: utils::get_next_unique_id(),
            bounds,
            label,
            font_size,
            texture: texture.unwrap_or_else(|| Self::default_texture().clone()),
            is_mouse_over: false,
            was_hovered: false,
            clicked: false,
        }
    }

    /// The background art every `Button` falls back to when `Button::new` is
    /// given `None` for `texture`. Loaded once and cloned per button —
    /// cloning a `Texture2D` is cheap, it's just a handle to the same GPU
    /// texture.
    fn default_texture() -> &'static Texture2D {
        static DEFAULT_TEXTURE: OnceLock<Texture2D> = OnceLock::new();
        DEFAULT_TEXTURE.get_or_init(|| {
            println!(
                "{LOG_PREFIX} loading default texture (assets/ui/button/button-background.png)..."
            );
            Texture2D::from_file_with_format(
                include_bytes!("../../../../assets/ui/button/button-background.png"),
                None,
            )
        })
    }

    /// Returns `true` if the mouse cursor is currently within the button's bounds.
    fn mouse_over_bounds(&self) -> bool {
        let (mx, my) = mouse_position();
        self.bounds.contains(Vec2::new(mx, my))
    }

    /// Returns `true` on the exact frame the button is clicked via the mouse —
    /// i.e. the cursor is over the button and the left mouse button was just
    /// pressed this frame. Edge-triggered (`is_mouse_button_pressed`), so this
    /// fires once per click rather than every frame the button is held down.
    fn is_clicked_via_mouse(&self) -> bool {
        self.mouse_over_bounds() && is_mouse_button_pressed(MouseButton::Left)
    }

    /// Returns `true` on the exact frame the cursor enters the button's
    /// bounds. Edge-triggered off last frame's `was_hovered`, mirroring
    /// `is_clicked_via_mouse`, so hover feedback (e.g. a sound) fires once
    /// per hover rather than every frame the cursor happens to linger.
    fn hover_started(&self) -> bool {
        self.is_mouse_over && !self.was_hovered
    }

    /// Returns `true` on the exact frame the cursor leaves the button's
    /// bounds — the counterpart to `hover_started`.
    fn hover_ended(&self) -> bool {
        !self.is_mouse_over && self.was_hovered
    }
}

impl StateUpdatable<()> for Button {
    /// Refreshes `is_mouse_over`/`clicked` from the current mouse state. This
    /// only updates the button's own state for rendering — it does not
    /// decide what a click *means*. Consumers read `poll_event` (via
    /// `Interactive`) after this to react to the click, keeping Button
    /// decoupled from whatever action it triggers.
    fn update_state(&mut self, _args: ()) {
        // Capture last frame's hover state before overwriting it — order
        // matters here: `hover_started`/`hover_ended` compare the two, so
        // `was_hovered` must still hold the *previous* frame's value when
        // `is_mouse_over` gets this frame's.
        self.was_hovered = self.is_mouse_over;
        self.is_mouse_over = self.mouse_over_bounds();
        self.clicked = self.is_clicked_via_mouse();
    }
}

impl Drawable for Button {
    fn draw(&self) {
        // Tints the background art for interaction feedback: darker while
        // held down, brighter on hover, unchanged (pure white multiplier)
        // otherwise.
        let tint = if self.clicked {
            Color::new(0.8, 0.8, 0.8, 1.0)
        } else if self.is_mouse_over {
            Color::new(1.2, 1.2, 1.2, 1.0)
        } else {
            WHITE
        };

        draw_texture_ex(
            &self.texture,
            self.bounds.x,
            self.bounds.y,
            tint,
            DrawTextureParams {
                dest_size: Some(self.bounds.size()),
                ..Default::default()
            },
        );

        let ts = measure_text(&self.label, None, self.font_size, 1.0);
        draw_text_ex(
            &self.label,
            self.bounds.x + (self.bounds.w - ts.width) / 2.0,
            self.bounds.y + self.bounds.h / 2.0 + ts.offset_y / 2.0,
            TextParams {
                font_size: self.font_size,
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

impl Interactive for Button {
    type Event = Option<ButtonEvents>;
    fn poll_event(&self) -> Self::Event {
        if self.clicked {
            return Some(ButtonEvents::Clicked);
        } else if self.hover_started() {
            return Some(ButtonEvents::HoverStarted);
        } else if self.hover_ended() {
            return Some(ButtonEvents::HoverEnded);
        } else if self.is_mouse_over {
            return Some(ButtonEvents::Hovering);
        }
        return None;
    }
}

impl SelfEventHandler for Button {
    fn handle_self_event(&mut self, event: Self::Event) {
        if let Some(ButtonEvents::Clicked) = event {
            // play audio click sound
            audio::play(AudioName::ButtonClick);
        } else if let Some(ButtonEvents::HoverStarted) = event {
            // play audio hover sound
            audio::play(AudioName::ButtonHover);
        }
    }
}
