//! Contains the logic concerning simulation objects.

use specs::prelude::*;

/// Represents a simulation.
pub struct Simulation {
    world: World
}


/// Represents an "acceleration" vector.
#[derive(Component, Debug)]
#[storage(VecStorage)]
struct Acceleration {
    x: f64,
    y: f64
}

/// Represents a "velocity" vector.
#[derive(Component, Debug)]
#[storage(VecStorage)]
struct Velocity {
    x: f64,
    y: f64
}
