//! Contains definitions for the various simulation entity components.

use crate::math::*;
use specs::{Component, Entity, VecStorage};
use std::collections::HashMap;


/// Represents the "camera" component.
#[derive(Clone, Component, Debug)]
#[storage(VecStorage)]
pub struct Camera {
    /// The field of view of the camera, in degrees.
    pub fov: u8,

    /// The angular position of the camera.
    pub orientation: Vector
}


/// Represents the "charge" component.
#[derive(Clone, Component, Debug)]
#[storage(VecStorage)]
pub struct Charge(pub f64);


/// Represents collision references to other entities.
#[derive(Clone, Component, Debug, Default)]
#[storage(VecStorage)]
pub struct Collisions(pub Vec<Entity>);


/// Represent the "description" component. All objects with this component
/// have a short description and long description.
#[derive(Clone, Component, Debug)]
#[storage(VecStorage)]
pub struct Description {
    /// The long description of the object.
    pub long_desc: String,

    /// The short description of the object.
    pub short_desc: String
}


/// Represents the "dynamics" component. All objects which inherit this
/// component are subject to the laws of newtonian dynamics.
#[derive(Clone, Component, Debug, Default)]
#[storage(VecStorage)]
pub struct Dynamics {
    /// The acceleration of the object.
    pub acceleration: Vector,
    
    /// The position of the object.
    pub position: Vector,

    /// The velocity of the object.
    pub velocity: Vector
}


/// Represents the "forces" component. This component keeps track of the various
/// forces acting on an object. The key of this `HashMap` corresponds to the
/// name of the force + the entity which imparted that force on this one.
#[derive(Clone, Component, Debug, Default)]
#[storage(VecStorage)]
pub struct Forces(pub HashMap<String, Vector>);


/// Represents the "lifetime" of an entity, which is the number of steps this
/// entity has existed.
#[derive(Clone, Component, Debug, Default)]
#[storage(VecStorage)]
pub struct Lifetime(pub u128);


/// Represents the "mass" component.
#[derive(Clone, Component, Debug)]
#[storage(VecStorage)]
pub struct Mass(pub f64);


/// Represents the "name" component.
#[derive(Clone, Component, Debug)]
#[storage(VecStorage)]
pub struct Name(pub String);


/// Represents the "orientation" component. All objects which inherit this
/// component are subject to things like angular acceleration.
#[derive(Clone, Component, Debug, Default)]
#[storage(VecStorage)]
pub struct Orientation {
    /// The angular acceleration of the object.
    pub angular_acceleration: Vector,

    /// The angular position (orientation) of the object.
    pub angular_position: Vector,

    /// The angular velocity of the object.
    pub angular_velocity: Vector
}


/// Represents the "physicality" component. All objects with physicality have a
/// bounding/size definition and may or may not be subject to collision detection.
#[derive(Clone, Component, Debug)]
#[storage(VecStorage)]
pub struct Physicality {
    /// The shape of the object.
    pub shape: Shape,

    /// Whether collision detection is enabled for this object.
    pub collisions_enabled: bool
}

/// Implements `std::default::Default` for `Physicality`.
impl std::default::Default for Physicality {
    fn default() -> Self { Physicality { shape: Shape::Point, collisions_enabled: true } }
}
