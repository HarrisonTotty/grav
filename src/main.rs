//! grav

#![feature(box_syntax, decl_macro, proc_macro_hygiene)]

#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate specs_derive;

pub mod cli;
pub mod ecs;
pub mod helper;
pub mod logging;
pub mod math;
pub mod output;

use specs::prelude::*;
use std::convert::TryInto;
use crate::ecs::systems::*;
use crate::ecs::resources::*;

/// The entrypoint of the program.
fn main() {
    // Parse CLI arguments.
    let args = cli::get_arguments();

    // Set-up logging.
    match logging::setup(
        args.value_of("log_file").unwrap(),
        args.value_of("log_level").unwrap(),
        args.value_of("log_mode").unwrap()
    ) {
        Ok(_)  => debug!("Initialized logging subsystem."),
        Err(e) => panic!("Unable to initialize logging subsystem - {}", e)
    }

    info!("Instantiating world...");
    let mut world = specs::World::new();

    info!("Registering components...");
    world.register::<ecs::components::Charge>();
    world.register::<ecs::components::Dynamics>();
    world.register::<ecs::components::Forces>();
    world.register::<ecs::components::Mass>();
    world.register::<ecs::components::Physicality>();

    info!("Instantiating resources...");
    world.insert(DeltaTime(0.5));
    world.insert(
        DynamicsLimits {
            maximum_acceleration: 2.0,
            maximum_position: 100.0,
            maximum_velocity: 10.0,
            minimum_acceleration: 0.0,
            minimum_position: 0.0,
            minimum_velocity: 0.0
        }
    );
    world.insert(ElectrostaticConstant(1.0));
    world.insert(GravitationalConstant::default());
    world.insert(OutputFile(args.value_of("output").unwrap().to_string()));

    info!("Building dispatcher...");
    let mut dispatcher = DispatcherBuilder::new()
        .with(
            ClearForces,
            "clear_forces",
            &[]
        )
        .with(
            HandleElectrostatics,
            "handle_electrostatics",
            &["clear_forces"]
        )
        .with(
            HandleGravity,
            "handle_gravity",
            &["clear_forces"]
        )
        .with(
            HandleForces,
            "handle_forces",
            &["handle_electrostatics", "handle_gravity"]
        )
        .with(
            HandleDynamics,
            "handle_dynamics",
            &["handle_forces"]
        )
        //.with(
        //    CollisionDetection,
        //    "collision_detection",
        //    &["handle_dynamics"]
        //)
        .with(
            WriteOutput,
            "write_output",
            &["handle_dynamics"]
        )
        .build();

    info!("Building entities...");
    helper::populate_entities(&mut world, 300);

    info!("Starting simulation...");
    let steps = args.value_of("steps").unwrap().parse::<u128>().unwrap();
    let pb = indicatif::ProgressBar::new(steps.try_into().unwrap());
    pb.set_prefix("Progress");
    pb.set_style(indicatif::ProgressStyle::default_bar()
                 .template("{prefix}: [ETA: {eta}] [{pos}/{len} ({percent}%)] {wide_bar}")
    );
    for step in 1..(steps + 1) {
        pb.inc(1);
        info!("Computing step {} of {}...", step, steps);
        dispatcher.dispatch(&mut world);
    }
    pb.finish();
}
