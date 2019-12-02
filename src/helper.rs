//! Helper functions, mainly for debugging.

use crate::ecs::components::*;
use crate::math::*;
use specs::prelude::*;

/// Populates the world with the specified set of entities.
pub fn populate_entities(world: &mut specs::World, num_entities: u32) {
    //use rand::Rng;
    //let mut rng = rand::thread_rng();
    for i in 0..num_entities {
        world.create_entity()
            .with(Charge(match i % 3 {
                0 => 0.0,
                1 => -1.0,
                _ => 1.0
            }))
            .with(Collisions::default())
            .with(
                Dynamics {
                    acceleration: Vector::default(),
                    position: Vector::random(1.0, 100.0),
                    velocity: Vector::random(0.0, 10.0)
                }
            )
            .with(Forces::default())
            .with(Lifetime::default())
            .with(Mass(1.0))
            .with(Physicality {
                collisions_enabled: true,
                shape: Shape::Sphere(1.0)
            })
            .build();
    }
}
