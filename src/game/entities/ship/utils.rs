pub mod user_input {
    use macroquad::prelude::*;

    /// Reads the arrow keys and returns a direction vector for this frame's
    /// movement input, normalized so diagonal movement isn't faster than
    /// moving along a single axis. Zero if no arrow key is held.
    pub fn movement_input() -> Vec2 {
        let mut dir = Vec2::ZERO;
        if is_key_down(KeyCode::Left) {
            dir.x -= 1.0;
        }
        if is_key_down(KeyCode::Right) {
            dir.x += 1.0;
        }
        if is_key_down(KeyCode::Up) {
            dir.y -= 1.0;
        }
        if is_key_down(KeyCode::Down) {
            dir.y += 1.0;
        }
        if dir != Vec2::ZERO {
            dir = dir.normalize();
        }
        dir
    }
}
