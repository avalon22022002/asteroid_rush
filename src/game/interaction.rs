/// Implemented by any game element that can report a discrete interaction
/// event for the current frame (click, toggle, collision, drag-release, etc).
///
/// This is deliberately separate from `StateUpdatable` — `StateUpdatable`
/// is for updating the element's own internal/visual state (hover, pressed,
/// animation progress); `Interactive` is for reporting *what happened* so
/// external code (like `Game`) can decide what to do about it.
pub trait Interactive {
    /// The event type this element can produce. Use an enum for elements
    /// with multiple possible events; `bool` is fine for simple "clicked
    /// or not" cases.
    type Event;

    /// Poll input for this frame and return what happened, if anything.
    /// Should not mutate anything outside `self`.
    fn poll_event(&self) -> Self::Event;
}
