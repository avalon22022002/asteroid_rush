use macroquad::texture::Texture2D;

const LOG_PREFIX: &str = "[animation]";

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

    /// Decodes each `(path, bytes)` pair as an image frame and builds an
    /// `Animation` from them. `path` is only used for load logging — pass
    /// the same path given to `include_bytes!` at the call site so log
    /// output (and the panic message, if the bytes aren't a valid image)
    /// names the actual file. `include_bytes!` bakes the file in at compile
    /// time, so a missing/renamed PNG fails the build instead of surfacing
    /// as a runtime error.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let alive_animation = Animation::load(
    ///     &[
    ///         (
    ///             "assets/ships/vanguard/alive_0.png",
    ///             include_bytes!("../../../assets/ships/vanguard/alive_0.png"),
    ///         ),
    ///         (
    ///             "assets/ships/vanguard/alive_1.png",
    ///             include_bytes!("../../../assets/ships/vanguard/alive_1.png"),
    ///         ),
    ///     ],
    ///     8.0,
    /// );
    /// ```
    pub fn load(frames: &[(&str, &[u8])], fps: f32) -> Self {
        println!(
            "{LOG_PREFIX} loading {} animation frame(s)...",
            frames.len()
        );

        let frames = frames
            .iter()
            .map(|(path, bytes)| {
                println!("{LOG_PREFIX} loading {path}...");
                Texture2D::from_file_with_format(bytes, None)
            })
            .collect();

        println!("{LOG_PREFIX} load complete");

        Self::new(frames, fps)
    }
}
