//! Defines ECS systems.

use specs::prelude::*;

use crate::component;
use crate::resource;

/// Struct on which basic dynamics are defined.
///
/// This system computes the new acceleration, position, and velocity components
/// of each particle based on each particle's net force and mass.
///
/// This system executes after the `Gravity` system.
pub struct Dynamics;

/// Implements the `specs::System` trait for the `Dynamics` struct.
impl<'a> System<'a> for Dynamics {
    type SystemData = (
        Read<'a, resource::DT>,
        ReadStorage<'a, component::Force>,
        ReadStorage<'a, component::Properties>,
        WriteStorage<'a, component::Acceleration>,
        WriteStorage<'a, component::Position>,
        WriteStorage<'a, component::Velocity>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (dt, force, props, acc, pos, vel) = data;
        for (force, acc, vel, pos) in (&force, &mut acc, &mut vel, &mut pos).join() {
            acc.x  = force.x / props.mass;
            acc.y  = force.y / props.mass;
            vel.x += acc.x * dt;
            vel.y += acc.y * dt;
            pos.x += vel.x * dt;
            pos.y += vel.y * dt;
        }
    }
}

/// Struct on which gravitational interactions are defined.
///
/// This system computes the force of gravity on a particle based on its
/// interactions with all of the other particles in the system. Since the force
/// between two particles is equal and opposite, we do some magic to ensure that
/// this calculation is only performed once per pair of particles.
///
/// This system will add (or update) the Force component of each particle.
pub struct Gravity;

/// Implements the `specs::System` trait for the `Gravity` struct.
impl<'a> System<'a> for Gravity {
    type SystemData = (
        Read<'a, resource::G>,
        ReadStorage<'a, component::Properties>,
        ReadStorage<'a, component::Position>,
        WriteStorage<'a, component::Force>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (g, props, pos, force) = data;
    }
}


