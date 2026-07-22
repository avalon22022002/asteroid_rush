/// Returns `(a, b)` reordered so the smaller value comes first, regardless
/// of which one was actually passed as "lower" or "upper".
pub fn ordered(a: f32, b: f32) -> (f32, f32) {
    (a.min(b), a.max(b))
}