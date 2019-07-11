//! # grav
//! A tui-based newtonian nbody simulation program written in rust.


// --------- Macros ----------
#[macro_use] extern crate log;
#[macro_use] extern crate specs_derive;
// ---------------------------


// ----- Custom Modules -----
mod component;
mod io;
mod resource;
mod simulation;
mod system;
// --------------------------


/// Handles the `load` subcommand.
fn handle_load<'a>(args: clap::ArgMatches<'a>) -> () {
    debug!("Handling \"load\" subcommand...");
}


/// Handles the `new` subcommand.
fn handle_new<'a>(args: clap::ArgMatches<'a>) -> () {
    debug!("Handling \"new\" subcommand...");
    if args.is_present("config") {
        let config_path = args.value_of("config").unwrap();
        debug!("Configuration Path: {}", config_path);
        
    }
}


/// The entry point of the program.
fn main() {
    // Parse CLI Arguments
    let args      = io::parse_arguments();
    let log_file  = args.value_of("log_file").unwrap();
    let log_level = args.value_of("log_level").unwrap();
    let log_mode  = args.value_of("log_mode").unwrap();

    // Setup logging
    match io::setup_logging(log_file, log_level, log_mode) {
        Ok(_) => debug!("Logging successfully initialized."),
        _     => panic!("Unable to initialize logging!")
    }

    // Handle subcommands
    match args.subcommand() {
        ("new", Some(a))  => handle_new(a.clone()),
        ("load", Some(a)) => handle_load(a.clone()),
        _                 => panic!("Unknown subcommand logic path!")
    }
}
