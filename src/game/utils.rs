use std::sync::atomic::{AtomicU64, Ordering};

/// Returns `(a, b)` reordered so the smaller value comes first, regardless
/// of which one was actually passed as "lower" or "upper".
pub fn ordered(a: f32, b: f32) -> (f32, f32) {
    (a.min(b), a.max(b))
}

// Backing counter for `get_next_unique_id`. Atomic (rather than a plain
// `static mut` or a counter threaded through game state) so ids stay unique
// and race-free even if id generation ever happens from multiple threads.
static NEXT_ID: AtomicU64 = AtomicU64::new(0);

/// Returns a process-wide unique id, starting at 0 and incrementing by 1
/// on every call. Ids are unique for the lifetime of the process but are
/// not stable across runs (the counter resets on restart) and are never reused.
pub fn get_next_unique_id()->u64{
    // `fetch_add` grabs the current number and bumps the counter in one
    // uninterruptible step, so two callers can never walk away with the same id.
    // `Relaxed` just means we're not using this counter to keep anything else
    // in sync across threads — we only care that every call gets a different
    // number, not exactly when each thread sees the update.
    return NEXT_ID.fetch_add(1, Ordering::Relaxed)
}