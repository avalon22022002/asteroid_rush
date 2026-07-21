use macroquad::prelude::*;

/// A type that exposes a human-readable name.
pub trait HasName {
    /// Returns this object's display name.
    fn name(&self) -> &str;
}

/// A type that exposes a unique identifier.
pub trait HasId {
    /// Returns this object's unique identifier.
    fn id(&self) -> u64;
}

/// A type that exposes an axis-aligned bounding box (`Rect`) for
/// collision checks, mouse interaction, and spatial queries.
///
/// The returned bounding box is expressed in world coordinates.
pub trait HasBoundingBox {
    /// Returns the axis-aligned bounding box of this object.
    fn bounding_box(&self) -> Rect;

    /// Returns `true` if this object's bounding box overlaps another object's
    /// bounding box.
    fn overlaps<T: HasBoundingBox>(&self, other: &T) -> bool {
        self.bounding_box().overlaps(&other.bounding_box())
    }

    /// Returns `true` if the given point is inside this object's bounding box.
    fn contains_point(&self, point: Vec2) -> bool {
        self.bounding_box().contains(point)
    }

    /// Returns `true` if the current mouse position is inside this object's
    /// bounding box.
    fn contains_current_mouse_position(&self) -> bool {
        let (x, y) = mouse_position();
        self.contains_point(Vec2::new(x, y))
    }
}

/// A type that can render its current state.
pub trait Drawable {
    /// Renders the object's current state. Called once per frame.
    fn draw(&self);
}

/// Base trait implemented by all game entities.
///
/// A game object must support collision boundaries, rendering, a name, and an id.
pub trait GameObject: HasBoundingBox + HasName + HasId + Drawable {}
