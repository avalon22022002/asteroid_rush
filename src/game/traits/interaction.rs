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

/// Implemented by an `Interactive` element that reacts to its *own* event
/// with a small, self-contained bit of feedback — a click sound, a flash, a
/// haptic pulse. It is not for app-level logic like page navigation or game
/// state transitions ("go to the game page when New Game is pressed"): that
/// depends on context the element doesn't have, and belongs with whatever
/// consumer already calls `poll_event` externally (e.g. the match in
/// `main.rs`). Bound to `Interactive` so it reuses the same `Event` type
/// rather than redeclaring it.
pub trait SelfEventHandler: Interactive {
    /// React to the event reported for this frame with feedback the element
    /// owns. Typically called right after `poll_event` (e.g. from within
    /// `StateUpdatable::update_state`), so implementors can assume `self`'s
    /// state is already up to date.
    fn handle_self_event(&mut self, event: Self::Event);
}
