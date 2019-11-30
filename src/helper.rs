//! Helper functions, mainly for debugging.

use crate::ecs::components::*;
use crate::math::*;
use specs::prelude::*;

/// Populates the world with the specified set of entities.
pub fn populate_entities(world: &mut specs::World, num_entities: u32) {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    for _i in 0..num_entities {
        let charge_bool: bool = rng.gen();
        world.create_entity()
            .with(Charge(match charge_bool {
                true => 1.0,
                _ => -1.0
            }))
            .with(
                Dynamics {
                    acceleration: Vector::default(),
                    position: Vector::random(0.0, 100.0),
                    velocity: Vector::random(0.0, 10.0)
                }
            )
            .with(Forces::default())
            .with(Mass(rng.gen_range(1.0, 10.0)))
            .with(Physicality {
                collisions_enabled: true,
                shape: Shape::Sphere(rng.gen_range(0.1, 1.0))
            })
            .build();
    }
}
