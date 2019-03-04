//! Contains ECS component definitions.

use specs::prelude::*;

/// Represents an "acceleration" vector.
#[derive(Component, Debug)]
#[storage(VecStorage)]
struct Acceleration {
    /// The "x" component of the vector.
    x: f64,
    /// The "y" component of the vector.
    y: f64
}

/// Represents a "force" vector.
#[derive(Component, Debug)]
#[storage(VecStorage)]
struct Force {
    /// The "x" component of the vector.
    x: f64,
    /// The "y" component of the vector.
    y: f64
}

/// Represents a "mass".
#[derive(Component, Debug)]
#[storage(VecStorage)]
struct Mass(f64);

/// Represents a "position" vector.
#[derive(Component, Debug)]
#[storage(VecStorage)]
struct Position {
    /// The "x" component of the vector.
    x: f64,
    /// The "y" component of the vector.
    y: f64
}

/// Represents a "velocity" vector.
#[derive(Component, Debug)]
#[storage(VecStorage)]
struct Velocity {
    /// The "x" component of the vector.
    x: f64,
    /// The "y" component of the vector.
    y: f64
}
