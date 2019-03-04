//! Contains handy functions for dealing with I/O.

use clap::{crate_authors, crate_description, crate_version};

/// Parses the command-line arguments, returning the collection of matches.
pub fn parse_arguments<'a>() -> clap::ArgMatches<'a> {
    let argument_parser = clap::App::new("grav")
        // General Information
        .about(crate_description!())
        .author(crate_authors!())
        .version(crate_version!())
        // General Parser Settings
        .settings(
            &[
                clap::AppSettings::ColoredHelp,
                clap::AppSettings::SubcommandRequiredElseHelp,
                clap::AppSettings::VersionlessSubcommands
            ]
        )
        // Subcommands
        .subcommand(clap::SubCommand::with_name("new")
                    .about("Runs a new simulation")
                    .arg(clap::Arg::with_name("config")
                         .help("Specifies the system configuration file to use for the simulation")
                         .long("config")
                         .short("c")
                         .value_name("FILE")
                    )
        )
        .subcommand(clap::SubCommand::with_name("load")
                    .about("Loads (plays-back) a previously saved simulation")
                    .arg(clap::Arg::with_name("simulation")
                         .help("Specifies the simulation file to load")
                         .value_name("FILE")
                    )
        )
        // Global Arguments
        .arg(clap::Arg::with_name("log_file")
             .default_value("grav.log")
             .env("GRAV_LOG_FILE")
             .help("Specifies the log file to write to")
             .long("log-file")
             .short("f")
             .value_name("FILE")
        )
        .arg(clap::Arg::with_name("log_level")
             .default_value("info")
             .env("GRAV_LOG_LEVEL")
             .help("Specifies the logging level of the program")
             .long("log-level")
             .possible_values(&["disabled", "error", "warning", "info", "debug", "trace"])
             .short("l")
             .value_name("LVL")
        )
        .arg(clap::Arg::with_name("log_mode")
             .default_value("append")
             .env("GRAV_LOG_MODE")
             .help("Specifies whether to append to, or overwrite, the specifies log file")
             .long("log-mode")
             .possible_values(&["append", "overwrite"])
             .short("M")
             .value_name("MODE")
        );
    return argument_parser.get_matches();
}


/// Sets-up logging for the program.
pub fn setup_logging(log_file: &str, log_level: &str, log_mode: &str) -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(
                format_args!(
                    "[{}] [{}] [{}] {}",
                    record.level(),
                    chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                    record.target(),
                    message
                )
            )
        })
        .level(match log_level {
            "disabled" => log::LevelFilter::Off,
            "error"    => log::LevelFilter::Error,
            "warning"  => log::LevelFilter::Warn,
            "info"     => log::LevelFilter::Info,
            "debug"    => log::LevelFilter::Debug,
            "trace"    => log::LevelFilter::Trace,
            _          => log::LevelFilter::Trace
        })
        .chain(std::fs::OpenOptions::new()
               .write(true)
               .create(true)
               .append(
                   match log_mode {
                       "append" => true,
                       _        => false
                   }
               )
               .open(log_file)?
        )
        .apply()?;
    return Ok(());
}
