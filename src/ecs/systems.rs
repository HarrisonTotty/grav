//! Contains definitions for various simulation systems.

use crate::ecs::components;
use crate::ecs::resources;
use crate::math::*;
use crate::output::*;
use specs::prelude::*;

/// Clears/resets the forces acting on all entities.
pub struct ClearForces;
impl<'a> System<'a> for ClearForces {
    type SystemData = WriteStorage<'a, components::Forces>;
    fn run(&mut self, mut forces: Self::SystemData) {
        debug!("Clearing forces...");
        for f in (&mut forces).join() {
            f.0 = std::collections::HashMap::new();
        }
    }
}


/// Detects collisions within the game world.
/// Objects which have collided are assigned a collision component.
pub struct CollisionDetection;
impl<'a> System<'a> for CollisionDetection {
    type SystemData = (
        Entities<'a>,
        Read<'a, resources::CollisionLimits>,
        ReadStorage<'a, components::Dynamics>,
        ReadStorage<'a, components::Orientation>,
        ReadStorage<'a, components::Physicality>,
        WriteStorage<'a, components::Collision>
    );
    fn run(&mut self, (entities, limits, dyns, ort, phys, mut collisions): Self::SystemData) {
        debug!("Detecting collisions...");
        for (i, (i_entity, i_dyns, _i_ort, i_phys)) in (&*entities, &dyns, &ort, &phys).join().enumerate() {
            if i_phys.collisions_enabled {
                for (j, (j_entity, j_dyns, _j_ort, j_phys)) in (&*entities, &dyns, &ort, &phys).join().enumerate() {
                    if i != j && j_phys.collisions_enabled {
                        trace!("DETECTING COLLISIONS: {:?} <-> {:?}", i_entity, j_entity);
                        let dist = (j_dyns.position - i_dyns.position).magnitude();
                        if dist < limits.maximum_detection_theshold {
                            if dist < limits.minimum_detection_theshold {
                                trace!("COLLISION: {:?} <-> {:?}", i_entity, j_entity);
                                if let Err(_msg) = collisions.insert(i_entity, components::Collision(j_entity)) {
                                    error!("Unable to assign collision to entity.");
                                }
                            } else {
                                match (i_phys.shape, j_phys.shape) {
                                    (Shape::Cuboid(_x1, _y1, _z1), Shape::Cuboid(_x2, _y2, _z2)) => {
                                    },
                                    (Shape::Cuboid(_x, _y, _z), Shape::Point) => {
                                    },
                                    (Shape::Cuboid(_x, _y, _z), Shape::Sphere(_r)) => {
                                    },
                                    (Shape::Sphere(_r), Shape::Cuboid(_x, _y, _z)) => {
                                    },
                                    (Shape::Sphere(r), Shape::Point) => {
                                        if dist - r <= 0.0 {
                                            trace!("COLLISION: {:?} <-> {:?}", i_entity, j_entity);
                                            if let Err(_msg) = collisions.insert(i_entity, components::Collision(j_entity)) {
                                                error!("Unable to assign collision to entity.");
                                            }
                                        }
                                    },
                                    (Shape::Sphere(r1), Shape::Sphere(r2)) => {
                                        if dist - (r1 + r2) <= 0.0 {
                                            trace!("COLLISION: {:?} <-> {:?}", i_entity, j_entity);
                                            if let Err(_msg) = collisions.insert(i_entity, components::Collision(j_entity)) {
                                                error!("Unable to assign collision to entity.");
                                            }
                                        }
                                    },
                                    (Shape::Point, Shape::Cuboid(_x, _y, _z)) => {
                                    },
                                    (Shape::Point, Shape::Point) => {
                                        // Points only collide when they are on top of each other, which should
                                        // be catched by `min_detection_theshold` above.
                                    },
                                    (Shape::Point, Shape::Sphere(r)) => {
                                        if dist - r <= 0.0 {
                                            trace!("COLLISION: {:?} <-> {:?}", i_entity, j_entity);
                                            if let Err(_msg) = collisions.insert(i_entity, components::Collision(j_entity)) {
                                                error!("Unable to assign collision to entity.");
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}


/// Handles the entities which have been detected as collided.
pub struct HandleCollisions;
impl<'a> System<'a> for HandleCollisions {
    type SystemData = (
        WriteStorage<'a, components::Collision>
    );
    fn run(&mut self, _data: Self::SystemData) {
        debug!("Handling collisions...");
    }
}


/// Handles updating the position and velocity of an entity from its
/// acceleration.
///
/// This system will also automatically truncate the various values according to
/// their limits, with the exception of "position", which will be toroidally
/// wrapped because our universe has periodic boundary conditions.
pub struct HandleDynamics;
impl<'a> System<'a> for HandleDynamics {
    type SystemData = (
        Read<'a, resources::DeltaTime>,
        Read<'a, resources::DynamicsLimits>,
        WriteStorage<'a, components::Dynamics>
    );
    fn run(&mut self, data: Self::SystemData) {
        debug!("Updating newtonian dynamics...");
        let (dt, limits, mut objects) = data;
        for obj in (&mut objects).join() {
            trace!(
                "OLD DYNAMICS: [{:?}, {:?}, {:?}]",
                &obj.acceleration,
                &obj.velocity,
                &obj.position
            );
            let acc_mag = obj.acceleration.magnitude();
            if acc_mag < limits.minimum_acceleration {
                obj.acceleration *= limits.minimum_acceleration / acc_mag;
            } else if acc_mag > limits.maximum_acceleration {
                obj.acceleration *= limits.maximum_acceleration / acc_mag;
            }
            obj.velocity += obj.acceleration * dt.0;
            let vel_mag = obj.velocity.magnitude();
            if vel_mag < limits.minimum_velocity {
                obj.velocity *= limits.minimum_velocity / vel_mag;
            } else if vel_mag > limits.maximum_velocity {
                obj.velocity *= limits.maximum_velocity / vel_mag;
            }
            obj.position += obj.velocity * dt.0;
            let pos_mag = obj.position.magnitude();
            if pos_mag < limits.minimum_position {
                obj.position *= limits.minimum_position / pos_mag;
            } else if pos_mag > limits.maximum_position {
                obj.position *= limits.maximum_position / pos_mag;
            }
            trace!(
                "NEW DYNAMICS: [{:?}, {:?}, {:?}]",
                &obj.acceleration,
                &obj.velocity,
                &obj.position
            );
        }
    }
}


/// Handles electrostatic interactions.
pub struct HandleElectrostatics;
impl<'a> System<'a> for HandleElectrostatics {
    type SystemData = (
        Entities<'a>,
        Read<'a, resources::ElectrostaticConstant>,
        ReadStorage<'a, components::Charge>,
        ReadStorage<'a, components::Dynamics>,
        WriteStorage<'a, components::Forces>
    );
    fn run(&mut self, (entities, k, charges, dynamics, mut forces): Self::SystemData) {
        debug!("Computing electrostatic interactions...");
        for (i, (i_entity, i_charge, i_dynamics)) in (&*entities, &charges, &dynamics).join().enumerate() {
            for (j, (j_entity, j_charge, j_dynamics)) in (&*entities, &charges, &dynamics).join().enumerate() {
                if let Some(i_forces) = forces.get_mut(i_entity) {
                    if i != j && !i_forces.0.contains_key(&format!("electrostatics:{:?}", j_entity)) {
                        trace!("COMPUTING ELECTROSTATICS: {:?} <-> {:?}", i_entity, j_entity);
                        let dvec = j_dynamics.position - i_dynamics.position;
                        let dmag = dvec.magnitude();
                        let es = dvec.direction() * ((-1.0 * k.0 * i_charge.0 * j_charge.0) / (dmag * dmag));
                        trace!("ELECTROSTATIC FORCE: {:?}", es);
                        i_forces.0.insert(
                            format!("electrostatics:{:?}", j_entity),
                            es
                        );
                        if let Some(j_forces) = forces.get_mut(j_entity) {
                            j_forces.0.insert(
                                format!("electrostatics:{:?}", i_entity),
                                -es
                            );
                        }
                    }
                } else {
                    trace!("{:?} does not have the \"Forces\" component.", i_entity);
                }
            }
        }
    }
}


/// Handles the translation of all forces into an acceleration vector.
pub struct HandleForces;
impl<'a> System<'a> for HandleForces {
    type SystemData = (
        ReadStorage<'a, components::Forces>,
        ReadStorage<'a, components::Mass>,
        WriteStorage<'a, components::Dynamics>
    );
    fn run(&mut self, (forces, masses, mut dynamics): Self::SystemData) {
        debug!("Computing net forces and acceleration...");
        for (f, m, d) in (&forces, &masses, &mut dynamics).join() {
            let net_force: Vector = f.0.values().sum();
            trace!("NET FORCE: {:?}", net_force);
            let acc = net_force / m.0;
            trace!("ACCELERATION: {:?}", acc);
            d.acceleration = acc;
        }
    }
}


/// Handles gravitational interactions.
pub struct HandleGravity;
impl<'a> System<'a> for HandleGravity {
    type SystemData = (
        Entities<'a>,
        Read<'a, resources::GravitationalConstant>,
        ReadStorage<'a, components::Dynamics>,
        ReadStorage<'a, components::Mass>,
        WriteStorage<'a, components::Forces>
    );
    fn run(&mut self, (entities, g, dynamics, masses, mut forces): Self::SystemData) {
        debug!("Computing newtonian gravitational interactions...");
        for (i, (i_entity, i_dynamics, i_mass)) in (&*entities, &dynamics, &masses).join().enumerate() {
            for (j, (j_entity, j_dynamics, j_mass)) in (&*entities, &dynamics, &masses).join().enumerate() {
                if let Some(i_forces) = forces.get_mut(i_entity) {
                    if i != j && !i_forces.0.contains_key(&format!("gravity:{:?}", j_entity)) {
                        trace!("COMPUTING GRAVITY: {:?} <-> {:?}", i_entity, j_entity);
                        let dvec = j_dynamics.position - i_dynamics.position;
                        let dmag = dvec.magnitude();
                        let grav = dvec.direction() * ((g.0 * i_mass.0 * j_mass.0) / (dmag * dmag));
                        trace!("FORCE OF GRAVITY: {:?}", grav);
                        i_forces.0.insert(
                            format!("gravity:{:?}", j_entity),
                            grav
                        );
                        if let Some(j_forces) = forces.get_mut(j_entity) {
                            j_forces.0.insert(
                                format!("gravity:{:?}", i_entity),
                                -grav
                            );
                        }
                    }
                } else {
                    trace!("{:?} does not have the \"Forces\" component.", i_entity);
                }
            }
        }
    }
}


/// Handles updating the angular position and velocity of an entity from its
/// angular acceleration. Note that the position vector is normalized to its
/// direction at the end.
pub struct HandleOrientation;
impl<'a> System<'a> for HandleOrientation {
    type SystemData = (
        Read<'a, resources::DeltaTime>,
        Read<'a, resources::OrientationLimits>,
        WriteStorage<'a, components::Orientation>
    );
    fn run(&mut self, data: Self::SystemData) {
        debug!("Updating angular dynamics (orientation)...");
        let (dt, limits, mut objects) = data;
        for obj in (&mut objects).join() {
            trace!(
                "OLD ORIENTATION: [{:?}, {:?}, {:?}]",
                &obj.angular_acceleration,
                &obj.angular_velocity,
                &obj.angular_position
            ); 
            let acc_mag = obj.angular_acceleration.magnitude();
            if acc_mag < limits.minimum_angular_acceleration {
                obj.angular_acceleration *= limits.minimum_angular_acceleration / acc_mag;
            } else if acc_mag > limits.maximum_angular_acceleration {
                obj.angular_acceleration *= limits.maximum_angular_acceleration / acc_mag;
            }
            obj.angular_velocity += obj.angular_acceleration * dt.0;
            let vec_mag = obj.angular_velocity.magnitude();
            if vec_mag < limits.minimum_angular_velocity {
                obj.angular_velocity *= limits.minimum_angular_velocity / vec_mag;
            } else if vec_mag > limits.maximum_angular_velocity {
                obj.angular_velocity *= limits.maximum_angular_velocity / vec_mag;
            }
            obj.angular_position += obj.angular_velocity * dt.0;
            obj.angular_position = obj.angular_position.direction();
            trace!(
                "NEW ORIENTATION: [{:?}, {:?}, {:?}]",
                &obj.angular_acceleration,
                &obj.angular_velocity,
                &obj.angular_position
            );
        }
    }
}


/// Writes simulation data to the specified output file.
pub struct WriteOutput;
impl<'a> System<'a> for WriteOutput {
    type SystemData = (
        Read<'a, resources::OutputFile>,
        ReadStorage<'a, components::Dynamics>,
        ReadStorage<'a, components::Mass>
    );
    fn run(&mut self, (output_file, dynamics, masses): Self::SystemData) {
        use std::io::Write;
        debug!("Writing output...");
        let mut output_entities: Vec<OutputEntity> = Vec::new();
        for (i_dynamics, i_mass) in (&dynamics, &masses).join() {
            output_entities.push(
                OutputEntity {
                    acceleration: i_dynamics.acceleration,
                    mass: i_mass.0,
                    position: i_dynamics.position,
                    velocity: i_dynamics.velocity
                }
            )
        }
        let entry = OutputEntry {
            step: 0,
            entities: output_entities
        };
        let yaml_string = format!("{}\n", serde_yaml::to_string(&entry).expect("Unable to serialize entry."));
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(&output_file.0)
            .expect("Unable to open output file.");
        file.write_all(yaml_string.as_bytes()).expect("Unable to write to output file.");
    }
}
