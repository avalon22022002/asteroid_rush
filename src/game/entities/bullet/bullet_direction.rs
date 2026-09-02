/// A bullet's travel direction: one of the four cardinal compass points, or
/// a precise custom angle for anything else (e.g. a spread shot).
///
/// Degrees follow the clock-face convention used by `Bullet`'s internal
/// `direction` field: 0 (`Up`) points up the screen, and angles increase
/// clockwise (`Right` = 90, `Down` = 180, `Left` = 270).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BulletDirection {
    Up,
    Right,
    Down,
    Left,
    /// An exact angle in degrees, using the same clock-face convention as
    /// the named variants (0 = up, clockwise-positive) — for angles the
    /// four named variants don't cover.
    Custom(f32),
}

impl BulletDirection {
    /// This direction as clock-face degrees (0 = up, clockwise-positive).
    pub fn degrees(self) -> f32 {
        match self {
            BulletDirection::Up => 0.0,
            BulletDirection::Right => 90.0,
            BulletDirection::Down => 180.0,
            BulletDirection::Left => 270.0,
            BulletDirection::Custom(degrees) => degrees,
        }
    }
}

impl Default for BulletDirection {
    /// Bullets default to travelling up the screen, matching a ship firing
    /// forward.
    fn default() -> Self {
        BulletDirection::Up
    }
}
