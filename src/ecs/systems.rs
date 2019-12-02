//! Contains definitions for various simulation systems.

use crate::ecs::components;
use crate::ecs::resources;
use crate::math::*;
use crate::output::*;
use specs::prelude::*;


/// Clears/resets the collisions between all entities.
pub struct ClearCollisions;
impl<'a> System<'a> for ClearCollisions {
    type SystemData = WriteStorage<'a, components::Collisions>;
    fn run(&mut self, mut collisions: Self::SystemData) {
        debug!("Clearing collisions...");
        for c in (&mut collisions).join() {
            c.0 = Vec::new();
        }
    }
}


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
        ReadStorage<'a, components::Physicality>,
        WriteStorage<'a, components::Collisions>
    );
    fn run(&mut self, (entities, limits, dyns, phys, mut collisions): Self::SystemData) {
        debug!("Detecting collisions...");
        for (i, (i_entity, i_dyns, i_phys)) in (&*entities, &dyns, &phys).join().enumerate() {
            if i_phys.collisions_enabled {
                for (j, (j_entity, j_dyns, j_phys)) in (&*entities, &dyns, &phys).join().enumerate() {
                    if let Some(i_collisions) = collisions.get_mut(i_entity) {
                        if i != j && j_phys.collisions_enabled && !i_collisions.0.contains(&j_entity) {
                           trace!("DETECTING COLLISIONS: {:?} <-> {:?}", i_entity, j_entity);
                           let dist = (j_dyns.position - i_dyns.position).magnitude();
                           if dist < limits.maximum_detection_theshold {
                               if dist < limits.minimum_detection_theshold {
                                   trace!("THRESHOLD COLLISION: {:?} <-> {:?}", i_entity, j_entity);
                                   i_collisions.0.push(j_entity);
                                   if let Some(j_collisions) = collisions.get_mut(j_entity) {
                                       j_collisions.0.push(i_entity);
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
                                               trace!("SPHERE-POINT COLLISION: {:?} <-> {:?}", i_entity, j_entity);
                                               i_collisions.0.push(j_entity);
                                               if let Some(j_collisions) = collisions.get_mut(j_entity) {
                                                   j_collisions.0.push(i_entity);
                                               }
                                           }
                                       },
                                       (Shape::Sphere(r1), Shape::Sphere(r2)) => {
                                           if dist - (r1 + r2) <= 0.0 {
                                               trace!("SPHERE-SPHERE COLLISION: {:?} <-> {:?}", i_entity, j_entity);
                                               i_collisions.0.push(j_entity);
                                               if let Some(j_collisions) = collisions.get_mut(j_entity) {
                                                   j_collisions.0.push(i_entity);
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
                                               trace!("POINT-SPHERE COLLISION: {:?} <-> {:?}", i_entity, j_entity);
                                               i_collisions.0.push(j_entity);
                                               if let Some(j_collisions) = collisions.get_mut(j_entity) {
                                                   j_collisions.0.push(i_entity);
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
}


/// Handles the entities which have been detected as collided.
pub struct HandleCollisions;
impl<'a> System<'a> for HandleCollisions {
    type SystemData = (
        Entities<'a>,
        Read<'a, LazyUpdate>,
        WriteStorage<'a, components::Charge>,
        WriteStorage<'a, components::Collisions>,
        WriteStorage<'a, components::Dynamics>,
        WriteStorage<'a, components::Mass>,
        WriteStorage<'a, components::Physicality>
    );
    fn run(&mut self, (entities, lazy_updater, mut all_charges, mut all_collisions, mut all_dynamics, mut all_masses, mut all_physicality): Self::SystemData) {
        debug!("Handling collisions...");
        for entity in (&*entities).join() {
            let collisions: Vec<Entity> = match all_collisions.get(entity) { Some(c) => c.0.clone(), _ => Vec::new() };
            if collisions.len() > 0 {
                let mut new_charge: f64 = match all_charges.get(entity) { Some(charge) => charge.0, _ => 0.0 };
                let mut new_mass: f64 = match all_masses.get(entity) { Some(mass) => mass.0, _ => 0.0 };
                let mut new_position: Vector = Vector::default();
                let mut new_velocity: Vector = Vector::default();
                let mut new_radius: f64 = 0.0;
                if let Some(dynamics) = all_dynamics.get(entity) {
                    new_position = dynamics.position;
                    new_velocity = dynamics.velocity;
                }
                if let Some(physicality) = all_physicality.get(entity) {
                    new_radius = match physicality.shape {
                        Shape::Sphere(r) => r / 2.0,
                        _ => 0.0
                    };
                }
                for other_entity in &collisions {
                    if let Some(other_charge) = all_charges.get(*other_entity) {
                        new_charge += other_charge.0;
                    }
                    if let Some(other_dynamics) = all_dynamics.get(*other_entity) {
                        new_position += (other_dynamics.position - new_position) / 2.0;
                        new_velocity += other_dynamics.velocity;
                    }
                    if let Some(other_mass) = all_masses.get(*other_entity) {
                        new_mass += other_mass.0;
                    }
                    if let Some(other_physicality) = all_physicality.get(*other_entity) {
                        if let Shape::Sphere(r) = other_physicality.shape {
                            new_radius += r / 2.0;
                        }
                    }
                    all_collisions.remove(*other_entity);
                    entities.delete(*other_entity).expect("Unable to delete other entity");
                }
                trace!("NEW CHARGE: {}", new_charge);
                trace!("NEW MASS: {}", new_mass);
                trace!("NEW POSITION: {:?}", new_position);
                trace!("NEW RADIUS: {}", new_radius);
                trace!("NEW VELOCITY: {:?}", new_velocity);
                let new_entity = entities.create();
                all_charges.insert(new_entity, components::Charge(new_charge)).expect("Unable to update charge");
                lazy_updater.insert(new_entity, components::Collisions::default());
                all_dynamics.insert(new_entity, components::Dynamics {
                    acceleration: Vector::default(),
                    position: new_position,
                    velocity: new_velocity
                }).expect("Unable to update dynamics");
                lazy_updater.insert(new_entity, components::Forces::default());
                lazy_updater.insert(new_entity, components::Lifetime::default());
                all_masses.insert(new_entity, components::Mass(new_mass)).expect("Unable to update mass");
                all_physicality.insert(new_entity, components::Physicality {
                    collisions_enabled: true,
                    shape: Shape::Sphere(new_radius)
                }).expect("Unable to update physicality");
                all_collisions.remove(entity);
                entities.delete(entity).expect("Unable to delete entity");
            }
        }
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
                obj.velocity = (-obj.velocity / 2.0);
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


/// Handles the splitting of particles into two.
pub struct HandleSplitting;
impl<'a> System<'a> for HandleSplitting {
    type SystemData = (
        Entities<'a>,
        Read<'a, LazyUpdate>,
        Read<'a, resources::SplittingSettings>,
        ReadStorage<'a, components::Lifetime>,
        WriteStorage<'a, components::Charge>,
        WriteStorage<'a, components::Dynamics>,
        WriteStorage<'a, components::Mass>,
        WriteStorage<'a, components::Physicality>
    );
    fn run(&mut self, (entities, lazy_updater, settings, lifetimes, mut all_charges, mut all_dynamics, mut all_masses, mut all_physicality): Self::SystemData) {
        debug!("Handling entity splitting...");
        for (entity, lifetime) in (&*entities, &lifetimes).join() {
            let mass: f64 = match all_masses.get(entity) { Some(m) => m.0, _ => 1.0 };
            let mut radius: f64 = 1.0;
            if let Some(physicality) = all_physicality.get(entity) {
                radius = match physicality.shape {
                    Shape::Sphere(r) => r,
                    _ => 1.0
                };
            }
            let mut split_factor: f64 = settings.maximum_lifetime as f64;
            if mass >= 10.0 {
                split_factor /= (mass / 10.0).floor();
            } else if mass <= -10.0 {
                split_factor /= (-mass / 10.0).floor();
            }
            if lifetime.0 > settings.minimum_lifetime && (lifetime.0 > settings.maximum_lifetime || (lifetime.0 as f64) > split_factor) {
                // Get the original component values.
                let charge: f64 = match all_charges.get(entity) { Some(c) => c.0, _ => 0.0 };
                let mut position = Vector::default();
                let mut velocity = Vector::default();
                if let Some(dynamics) = all_dynamics.get(entity) {
                    position = dynamics.position;
                    velocity = dynamics.velocity;
                }
                // Setup the two new particles.
                let p1 = entities.create();
                let p2 = entities.create();
                if charge == 0.0 {
                    all_charges.insert(p1, components::Charge(-1.0)).expect("Unable to set charge");
                    all_charges.insert(p2, components::Charge(1.0)).expect("Unable to set charge");
                } else {
                    all_charges.insert(p1, components::Charge((charge / 2.0).floor())).expect("Unable to set charge");
                    all_charges.insert(p2, components::Charge((charge / 2.0).ceil())).expect("Unable to set charge");
                }
                all_masses.insert(p1, components::Mass(mass / 2.0)).expect("Unable to set mass");
                all_masses.insert(p2, components::Mass(mass / 2.0)).expect("Unable to set mass");
                all_dynamics.insert(p1, components::Dynamics {
                    acceleration: Vector::default(),
                    position: position + (settings.separation_multiplier * radius),
                    velocity: velocity * settings.velocity_multiplier
                }).expect("Unable to set dynamics.");
                all_dynamics.insert(p2, components::Dynamics {
                    acceleration: Vector::default(),
                    position: position - (settings.separation_multiplier * radius),
                    velocity: -(velocity * settings.velocity_multiplier)
                }).expect("Unable to set dynamics.");
                all_physicality.insert(p1, components::Physicality {
                    collisions_enabled: true,
                    shape: Shape::Sphere(radius)
                }).expect("Unable to set physicality");
                all_physicality.insert(p2, components::Physicality {
                    collisions_enabled: true,
                    shape: Shape::Sphere(radius)
                }).expect("Unable to set physicality");
                lazy_updater.insert(p1, components::Collisions::default());
                lazy_updater.insert(p2, components::Collisions::default());
                lazy_updater.insert(p1, components::Forces::default());
                lazy_updater.insert(p2, components::Forces::default());
                lazy_updater.insert(p1, components::Lifetime::default());
                lazy_updater.insert(p2, components::Lifetime::default());
                entities.delete(entity).expect("Unable to delete entity");
            }
        }
    }
}


/// Updates the lifetime of all entities.
pub struct UpdateLifetimes;
impl<'a> System<'a> for UpdateLifetimes {
    type SystemData = WriteStorage<'a, components::Lifetime>;
    fn run(&mut self, mut lifetimes: Self::SystemData) {
        debug!("Updating entity lifetimes...");
        for lifetime in (&mut lifetimes).join() {
            lifetime.0 += 1;
        }
    }
}


/// Writes simulation data to the specified output file.
pub struct WriteOutput;
impl<'a> System<'a> for WriteOutput {
    type SystemData = (
        Read<'a, resources::OutputFile>,
        ReadStorage<'a, components::Charge>,
        ReadStorage<'a, components::Dynamics>,
        ReadStorage<'a, components::Mass>
    );
    fn run(&mut self, (output_file, charges, dynamics, masses): Self::SystemData) {
        use std::io::Write;
        debug!("Writing output...");
        let mut output_entities: Vec<OutputEntity> = Vec::new();
        for (i_charge, i_dynamics, i_mass) in (&charges, &dynamics, &masses).join() {
            let oe = OutputEntity {
                acceleration: i_dynamics.acceleration,
                charge: i_charge.0,
                mass: i_mass.0,
                position: i_dynamics.position,
                velocity: i_dynamics.velocity
            };
            trace!("OUTPUT ENTITY: {:?}", oe);
            output_entities.push(oe);
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
