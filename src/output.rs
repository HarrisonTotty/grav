//! Defines structs used in specifying output files.

use crate::math::*;

/// Represents a specific entry in the output file.
#[derive(Serialize, Debug)]
pub struct OutputEntry {
    /// The time step this entry represents.
    pub step: u128,

    /// The collection of entities.
    pub entities: Vec<OutputEntity>
}

/// Represents an entity, as defined in the output file.
#[derive(Serialize, Debug)]
pub struct OutputEntity {
    /// The current acceleration of this entity.
    pub acceleration: Vector,

    /// The charge of the entity.
    pub charge: f64,

    /// The mass of the entity.
    pub mass: f64,

    /// The current position of this entity.
    pub position: Vector,

    /// The current velocity of this entity.
    pub velocity: Vector
}
