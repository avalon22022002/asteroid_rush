/// A type that can render its current state.
pub trait Drawable {
    /// Renders the object's current state. Called once per frame.
    fn draw(&self);
}

/// A type whose internal state can be advanced over time, and which can also render itself.
pub trait StateUpdatable<T>: Drawable {
    /// Advances the object's state using the given update data. Called once per frame, before draw.
    fn update_state(&mut self, data: T);
}