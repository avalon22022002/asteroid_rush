use macroquad::{math::{Rect, Vec2}, texture::Texture2D};

/// A sequence of sprite frames played back at a fixed rate. Repeats by
/// default; call `.play_once()` at construction to freeze on the last frame
/// instead.
#[derive(Debug, Clone)]
pub struct Animation {
    frames: Vec<Texture2D>,
    scale: Vec2,
    /// Optional sub-rect of each frame to draw, in source-texture pixels.
    /// `None` draws the whole frame; a crop strips transparent padding so the
    /// art fills its `scale` box (see `SpriteBounds`).
    crop: Option<Rect>,
    /// Seconds each frame is shown for (`1.0 / fps`).
    frame_duration: f32,
    /// Seconds accumulated since the current frame started.
    elapsed: f32,
    current: usize,
    repeats: bool,
}

impl Animation {
    /// `frames` is cloned in — cheap, since `Texture2D` is just a handle to
    /// a GPU texture, not a deep copy. `scale` is the on-screen draw size
    /// shared by every frame. `fps` is how many frames to show per second.
    /// `crop` is an optional sub-rect (source-texture pixels) drawn from each
    /// frame, e.g. to strip transparent padding; `None` draws the whole frame.
    pub fn new(frames: &[Texture2D], scale: Vec2, fps: f32, crop: Option<Rect>) -> Self {
        Self {
            frames: frames.to_vec(),
            scale,
            crop,
            frame_duration: 1.0 / fps,
            elapsed: 0.0,
            current: 0,
            repeats: true,
        }
    }

    /// Marks this animation as playing once: the full animation plays
    /// through, then the end frame is displayed forever, instead of looping
    /// back to the first frame. Use for animations that shouldn't repeat —
    /// e.g. a ship's death explosion, which should play through once and
    /// hold on the settled-debris end frame, rather than replaying the
    /// explosion animation on a loop.
    pub fn play_once(mut self) -> Self {
        self.repeats = false;
        self
    }

    /// The crop rect, if one was set — pass straight into
    /// `DrawTextureParams::source`.
    pub fn frame_crop(&self) -> Option<Rect> {
        self.crop
    }

    /// Pixel size of a frame's drawn region: the `crop` if set, else the full
    /// current-frame texture.
    pub fn content_size(&self) -> Vec2 {
        match self.crop {
            Some(r) => r.size(),
            None => {
                let f = self.current_frame();
                Vec2::new(f.width(), f.height())
            }
        }
    }

    /// Advances playback by `dt` seconds. When repeating is enabled, loops
    /// back to the first frame once the current frame's duration elapses.
    /// Otherwise, stops on the final frame.
    pub fn advance(&mut self, dt: f32) {
        if self.frames.len() <= 1 {
            return;
        }

        self.elapsed += dt;

        while self.elapsed >= self.frame_duration {
            self.elapsed -= self.frame_duration;

            if self.current + 1 >= self.frames.len() {
                if self.repeats {
                    // End of sequence — loop back to the start.
                    self.current = 0;
                } else {
                    // End of sequence — hold on the last frame since repeat is disabled.
                    self.current = self.frames.len() - 1;
                    self.elapsed = 0.0;
                    break;
                }
            } else {
                // Mid-sequence — advance to the next frame.
                self.current += 1;
            }
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
