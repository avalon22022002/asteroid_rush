use macroquad::{audio, prelude::*};

use crate::game::{
    asset_repository::{
        traits::Singleton,
        sprite_repository::{traits::{SpriteTextures, SpriteBounds}, SpriteRepository, ButtonV1Textures},
        audio_repository::{
            AudioRepository,
            traits::AudioClips,
            button_click::ButtonClickSound,
            button_hover::ButtonHoverSound,
        },
    },
    entities::animation::Animation,
    interaction::{Interactive, SelfEventHandler},
    object::HasId,
    rendering::{Drawable, StateUpdatable},
    ui::components::utils::additive_glow,
    utils,
};

const LOG_PREFIX: &str = "[button]";

/// The visual style of a `Button`: its background art (and later, any
/// per-style tweaks like tint or sound). Callers pick a `ButtonKind`; the
/// mapping to the underlying texture asset stays internal to this module, so
/// swapping art or adding styles never touches call sites.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonKind {
    /// The standard blue sci-fi menu button used across the game's pages.
    Basic,
}

impl ButtonKind {
    /// The background texture this style draws.
    fn texture(&self) -> ButtonV1Textures {
        match self {
            ButtonKind::Basic => ButtonV1Textures::BasicScifiV1,
        }
    }
}

impl SpriteBounds for ButtonKind {
    /// Delegates to the backing texture so callers can size a button by its
    /// art's aspect ratio without naming the texture enum directly.
    fn content_bounds(&self) -> Rect {
        self.texture().content_bounds()
    }
}

/// # Example
///
/// ```no_run
/// use asteroid_rush::game::{
///     interaction::{Interactive, SelfEventHandler},
///     rendering::{Drawable, StateUpdatable},
///     ui::components::button::{Button, ButtonEvents, ButtonKind},
/// };
/// use macroquad::prelude::*;
///
/// async fn demo() {
///     let mut new_game_button = Button::new(
///         Rect::new(20.0, 100.0, 200.0, 48.0),
///         "New Game".to_string(),
///         30,
///         ButtonKind::Basic,
///     );
///
///     loop {
///         // Refresh hover/clicked state from this frame's input.
///         new_game_button.update_state(());
///         new_game_button.draw();
///
///         // Let the button react to its own event first — self-contained
///         // feedback it owns, like playing the click sound. `event` is
///         // `Copy`, so reading it here doesn't stop the consumer below from
///         // reading it too.
///         let event = new_game_button.poll_event();
///         new_game_button.handle_self_event(event);
///
///         // Then the consumer decides what the click *means* for the app.
///         // That's custom app logic (page navigation, game state, ...), which
///         // `SelfEventHandler` deliberately stays out of — the button has no
///         // notion of "pages".
///         if let Some(ButtonEvents::Clicked) = event {
///             println!("New Game clicked — switch to the game page");
///         }
///
///         next_frame().await;
///     }
/// }
/// ```
pub struct Button {
    id: u64,
    bounds: Rect,
    label: String,
    font_size: u16,
    kind: ButtonKind,
    animation: Animation,
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
    pub fn new(bounds: Rect, label: String, font_size: u16, kind: ButtonKind) -> Self {
        let button_sprites = &SpriteRepository::get_instance().button_v1_sprite;
        Self {
            id: utils::get_next_unique_id(),
            bounds,
            label,
            font_size,
            kind,
            animation: Animation::new(
                button_sprites.get_textures_for(&kind.texture()),
                bounds.size(),
                1.0,
                None,
            ),
            is_mouse_over: false,
            was_hovered: false,
            clicked: false,
        }
    }

    /// This button's on-screen position and size.
    pub fn bounds(&self) -> Rect {
        self.bounds
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

    /// Re-draws the button art over itself additively so only the bright blue
    /// panel lights up on hover (see `additive_glow`). The blue tint biases the
    /// added light toward blue; a faint sine pulse keeps it alive without
    /// visibly flickering.
    fn draw_hover_glow(&self) {
        // Glow intensity oscillating between 0.55 and 0.65 for a slow, steady pulse.
        let pulse = 0.60 + 0.05 * (get_time() as f32 * 3.0).sin();
        additive_glow::draw(
            self.animation.current_frame(),
            self.bounds,
            Color::new(0.4, 0.7, 1.0, pulse),
            Some(self.kind.content_bounds()),
        );
    }
}

impl Default for Button {
    fn default() -> Self {
        Button::new(Rect::new(0.0, 0.0, 50.0, 25.0), "Default Button".to_string(), 10, ButtonKind::Basic)
    }
}

impl StateUpdatable<()> for Button {
    /// Refreshes `is_mouse_over`/`clicked` from the current mouse state. This
    /// only updates the button's own state
    fn update_state(&mut self, _args: ()) {
        // Save the old hover state into `was_hovered` *before* refreshing
        // `is_mouse_over`, so we keep both last frame's and this frame's
        // values. `hover_started`/`hover_ended` detect edges by comparing
        // the two, which only works when `was_hovered` holds the previous
        // value and `is_mouse_over` holds the current one.
        self.was_hovered = self.is_mouse_over;
        self.is_mouse_over = self.mouse_over_bounds();
        self.clicked = self.is_clicked_via_mouse();
    }
}

impl Drawable for Button {
    fn draw(&self) {
        // Per-pixel color multiplier applied to the background art. `WHITE`
        // (1,1,1,1) leaves the art untouched; the `0.8` gray darkens it while
        // the button is held down, giving a "pressed in" look. Hover doesn't
        // need a branch here — the additive glow pass below handles that cue.
        let tint = if self.clicked {
            Color::new(0.8, 0.8, 0.8, 1.0)
        } else {
            WHITE
        };

        draw_texture_ex(
            self.animation.current_frame(),
            self.bounds.x,
            self.bounds.y,
            tint,
            DrawTextureParams {
                dest_size: Some(self.bounds.size()),
                // Crop the texture's transparent margins so the button art fills
                // the bounds, keeping layout gaps and the hit area honest.
                source: Some(self.kind.content_bounds()),
                ..Default::default()
            },
        );

        // Additive glow pass on top of the base art, so only the bright blue
        // panel lights up on hover.
        if self.is_mouse_over && !self.clicked {
            self.draw_hover_glow();
        }

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
        None
    }
}

impl SelfEventHandler for Button {
    fn handle_self_event(&mut self, event: Self::Event) {
        if let Some(ButtonEvents::Clicked) = event {
            // play audio click sound
            audio::play_sound_once(
                AudioRepository::get_instance()
                    .button_click_sounds
                    .get_clip_for(&ButtonClickSound::Basic)
                    .unwrap_or_else(|| panic!("{LOG_PREFIX} button click sound not loaded")),
            );
        } else if let Some(ButtonEvents::HoverStarted) = event {
            // play audio hover sound
            audio::play_sound_once(
                AudioRepository::get_instance()
                    .button_hover_sounds
                    .get_clip_for(&ButtonHoverSound::Basic)
                    .unwrap_or_else(|| panic!("{LOG_PREFIX} button hover sound not loaded")),
            );
        }
    }
}
