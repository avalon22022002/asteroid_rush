use macroquad::texture::Texture2D;

/// A sequence of sprite frames played back at a fixed rate, looping.
pub struct Animation {
    frames: Vec<Texture2D>,
    frame_duration: f32,
    elapsed: f32,
    current: usize,
}

impl Animation {
    /// `fps` is how many frames to show per second.
    pub fn new(frames: Vec<Texture2D>, fps: f32) -> Self {
        Self {
            frames,
            frame_duration: 1.0 / fps,
            elapsed: 0.0,
            current: 0,
        }
    }

    /// Advances playback by `dt` seconds, looping back to the first frame
    /// once the current one's duration elapses.
    pub fn advance(&mut self, dt: f32) {
        if self.frames.len() <= 1 {
            return;
        }
        self.elapsed += dt;
        while self.elapsed >= self.frame_duration {
            self.elapsed -= self.frame_duration;
            self.current = (self.current + 1) % self.frames.len();
        }
    }

    /// Restarts playback from the first frame. Call this when switching
    /// into an animation (e.g. alive -> dead) so it doesn't resume
    /// mid-cycle from whatever frame the previous animation left off on.
    pub fn restart(&mut self) {
        self.current = 0;
        self.elapsed = 0.0;
    }

    pub fn current_frame(&self) -> &Texture2D {
        &self.frames[self.current]
    }

    /// Decodes each byte slice as an image frame and builds an `Animation`
    /// from them. Panics if a frame's bytes aren't a valid image — pair with
    /// `include_bytes!` at the call site so a missing/renamed PNG fails the
    /// build instead of surfacing as a runtime error.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let alive_animation = Animation::load(
    ///     &[
    ///         include_bytes!("../../../assets/ships/vanguard/alive_0.png"),
    ///         include_bytes!("../../../assets/ships/vanguard/alive_1.png"),
    ///     ],
    ///     8.0,
    /// );
    /// ```
    pub fn load(frame_bytes: &[&[u8]], fps: f32) -> Self {
        let frames = frame_bytes
            .iter()
            .map(|bytes| Texture2D::from_file_with_format(bytes, None))
            .collect();
        Self::new(frames, fps)
    }
}
