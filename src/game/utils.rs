use std::sync::atomic::{AtomicU64, Ordering};
use macroquad::math::Vec2;

/// A min/max pair, used instead of a tuple so callers don't have to remember
/// `.0` vs `.1` to know which side is which.
pub struct MinMax<T> {
    pub min: T,
    pub max: T,
}

/// Returns `(a, b)` reordered so the smaller value comes first, regardless
/// of which one was actually passed as "lower" or "upper".
pub fn ordered(a: f32, b: f32) -> (f32, f32) {
    (a.min(b), a.max(b))
}

/// Computes a `Vec2` size for a fixed height, using `aspect_ratio` (width/height)
/// to derive the width.
///
/// - Use when height is the constrained dimension (e.g. fits a fixed-height slot)
///   and width should follow to match the art's proportions.
/// - Prevents stretching: scaling width and height independently distorts the art.
pub fn aspect_size_from_fixed_height(height: f32, aspect_ratio: f32) -> Vec2 {
    Vec2::new(height * aspect_ratio, height)
}

/// Computes a `Vec2` size for a fixed width, using `aspect_ratio` (width/height)
/// to derive the height.
///
/// - Use when width is the constrained dimension (e.g. fits a fixed-width slot)
///   and height should follow to match the art's proportions.
/// - Prevents stretching: scaling width and height independently distorts the art.
pub fn aspect_size_from_fixed_width(width: f32, aspect_ratio: f32) -> Vec2 {
    Vec2::new(width, width / aspect_ratio)
}

// Backing counter for `get_next_unique_id`. Atomic (rather than a plain
// `static mut` or a counter threaded through game state) so ids stay unique
// and race-free even if id generation ever happens from multiple threads.
static NEXT_ID: AtomicU64 = AtomicU64::new(0);

/// Returns a process-wide unique id, starting at 0 and incrementing by 1
/// on every call. Ids are unique for the lifetime of the process but are
/// not stable across runs (the counter resets on restart) and are never reused.
pub fn get_next_unique_id() -> u64 {
    // `fetch_add` grabs the current number and bumps the counter in one
    // uninterruptible step, so two callers can never walk away with the same id.
    // `Relaxed` just means we're not using this counter to keep anything else
    // in sync across threads — we only care that every call gets a different
    // number, not exactly when each thread sees the update.
    NEXT_ID.fetch_add(1, Ordering::Relaxed)
}

/// Returns a random value within `[min, max]` with a directional bias.
///
/// `bias` is clamped to the range `[-1.0, 1.0]`.
///
/// - `bias = -1.0` → always `min`
/// - `bias =  0.0` → uniform random value
/// - `bias =  1.0` → always `max`
///
/// Values between `-1.0` and `1.0` smoothly pull the result
/// toward the corresponding side of the range.
///
/// # Example
///
/// For `min = 0.0` and `max = 100.0`:
///
/// ```text
/// bias = -1.0  → 0.0   (always min)
/// bias = -0.5  → 23.7  (random value pulled toward min)
/// bias =  0.0  → 68.4  (uniform random value)
/// bias =  0.5  → 84.2  (random value pulled toward max)
/// bias =  1.0  → 100.0 (always max)
///
/// Note: the exact random values will differ on every call.
/// ```
pub fn biased_random_in_range(min_max: MinMax<f32>, bias: f32) -> f32 {
    let (min, max)=(min_max.min, min_max.max);
    // Keep bias within the expected [-1, 1] range.
    let bias = bias.clamp(-1.0, 1.0);
    // Start with a uniformly distributed random value across the full range.
    // This is our baseline before any bias is applied.
    let random =  macroquad::rand::gen_range(min,max);
    if bias > 0.0 {
        // Positive bias: shift `random` toward `max`.
        // `(max - random)` is the remaining distance to max, and `bias`
        // (0..1) controls how much of that distance we close.
        // At bias = 1.0, this fully collapses the result to `max`
        random + (max - random) * bias
    } else {
        // Negative bias: same idea, but shift toward `min` instead.
        // `-bias` flips the negative bias into a positive weight (0..1)
        // since `bias` is <= 0 here.
        // At bias = -1.0, this fully collapses the result to `min`.
        random + (min - random) * (-bias)
    }
}
