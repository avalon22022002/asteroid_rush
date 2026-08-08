use macroquad::{math::{Vec2}, texture::Texture2D};

const LOG_PREFIX: &str = "[animation]";

/// A sequence of sprite frames played back at a fixed rate, looping.
pub struct Animation {
    frames: Vec<Texture2D>,
    scale: Vec2,
    frame_duration: f32,
    elapsed: f32,
    current: usize,
}

impl Animation {
    /// `frames` is cloned in — cheap, since `Texture2D` is just a handle to
    /// a GPU texture, not a deep copy. `scale` is the on-screen draw size
    /// shared by every frame. `fps` is how many frames to show per second.
    pub fn new(frames: &[Texture2D], scale: Vec2, fps: f32) -> Self {
        Self {
            frames: frames.to_vec(),
            scale,
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
    
    pub fn frame_scale(&self) -> &Vec2 {
        &self.scale
    }
}
