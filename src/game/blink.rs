/// Toggles visibility on/off at a fixed rate for a limited window — e.g. a
/// ship or asteroid flashing briefly after taking damage. Starts inactive
/// (always visible); call `trigger` to start a blink window.
#[derive(Debug, Clone, Copy)]
pub struct Blink {
    /// Seconds per on/off phase.
    interval: f32,
    /// Seconds left in the current blink window; `<= 0.0` means inactive.
    remaining: f32,
    /// Seconds accumulated within the current phase.
    phase_elapsed: f32,
    visible: bool,
}

impl Blink {
    /// `interval` is how long each visible/invisible phase lasts, in seconds.
    pub fn new(interval: f32) -> Self {
        Self {
            interval,
            remaining: 0.0,
            phase_elapsed: 0.0,
            visible: true,
        }
    }

    /// Starts (or restarts) a blink window lasting `duration` seconds.
    pub fn trigger(&mut self, duration: f32) {
        self.remaining = duration;
        self.phase_elapsed = 0.0;
        self.visible = true;
    }

    /// Advances the blink window by `dt` seconds, toggling visibility every
    /// `interval` seconds until the window runs out, then settles visible.
    pub fn advance(&mut self, dt: f32) {
        if self.remaining <= 0.0 {
            return;
        }

        self.remaining -= dt;
        if self.remaining <= 0.0 {
            self.visible = true;
            return;
        }

        self.phase_elapsed += dt;
        while self.phase_elapsed >= self.interval {
            self.phase_elapsed -= self.interval;
            self.visible = !self.visible;
        }
    }

    /// Whether the entity should be drawn this frame.
    pub fn is_visible(&self) -> bool {
        self.visible
    }
}
