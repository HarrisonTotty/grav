//! Contains ECS component definitions.

use specs::prelude::*;

/// Represents an "acceleration" vector.
#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Acceleration {
    /// The "x" component of the vector.
    pub x: f64,
    /// The "y" component of the vector.
    pub y: f64
}

/// Represents a "force" vector.
#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Force {
    /// The "x" component of the vector.
    pub x: f64,
    /// The "y" component of the vector.
    pub y: f64
}

/// Represents non-dynamic properties such as "mass" or "charge".
#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Properties {
    /// The "mass" of the entity.
    pub mass: f64
}

/// Represents a "position" vector.
#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Position {
    /// The "x" component of the vector.
    pub x: f64,
    /// The "y" component of the vector.
    pub y: f64
}

/// Represents a "velocity" vector.
#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Velocity {
    /// The "x" component of the vector.
    pub x: f64,
    /// The "y" component of the vector.
    pub y: f64
}
