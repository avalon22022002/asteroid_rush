/// A type that can render its current state.
pub trait Drawable {
    /// Renders the object's current state. Called once per frame.
    fn draw(&self);
}
