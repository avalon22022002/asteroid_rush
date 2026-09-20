/// Outcome of a `Damageable::take_damage` call — whether the entity
/// survived the hit or was destroyed by it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DamageResult {
    Alive,
    Destroyed,
}

/// A type that can take damage and report whether it survived.
pub trait Damageable {
    /// Reduces this entity's health by `amount`, returning whether the hit
    /// destroyed it.
    fn take_damage(&mut self, amount: u32) -> DamageResult;
}
