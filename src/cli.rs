//! Contains useful functions pertaining to setting-up and maintaining CLI arguments.

/// Parses the command-line arguments passed to the program, returning a
/// collection of matches.
pub fn get_arguments<'a>() -> clap::ArgMatches<'a> {
    use clap:: {
        crate_authors,
        crate_description,
        crate_name,
        crate_version
    };
    let argument_parser = clap::App::new(crate_name!())
        .about(crate_description!())
        .author(crate_authors!())
        .help_message("Displays help and usage information.")
        .version(crate_version!())
        .version_message("Displays version information.")
        .arg(clap::Arg::with_name("data_dir")
             .default_value("data")
             .env("GRAV_DATA_DIR")
             .help("Specifies the directory from which to load simulation data.")
             .long("--data-dir")
             .short("-d")
             .value_name("DIR")
        )
        .arg(clap::Arg::with_name("log_file")
             .default_value("grav.log")
             .env("GRAV_LOG_FILE")
             .help("Specifies the log file to write simulation events to.")
             .long("--log-file")
             .short("-f")
             .value_name("FILE")
        )
        .arg(clap::Arg::with_name("log_level")
             .default_value("info")
             .env("GRAV_LOG_LEVEL")
             .help("Specifies the logging level of the program.")
             .long("--log-level")
             .possible_values(&[
                 "disabled",
                 "error",
                 "warning",
                 "info",
                 "debug",
                 "trace"
             ])
             .short("-l")
             .value_name("LVL")
        )
        .arg(clap::Arg::with_name("log_mode")
             .default_value("overwrite")
             .env("GRAV_LOG_MODE")
             .help("Specifies whether to append to, or overwrite, the specified log file.")
             .long("--log-mode")
             .possible_values(&[
                 "append",
                 "overwrite"
             ])
             .short("-m")
             .value_name("MODE")
        )
        .arg(clap::Arg::with_name("output")
             .default_value("output.yaml")
             .env("GRAV_OUTPUT")
             .help("Specifies the simulation output file.")
             .long("--output")
             .short("-o")
             .value_name("FILE")
        )
        .arg(clap::Arg::with_name("steps")
             .default_value("1000")
             .env("GRAV_STEPS")
             .help("Specifies the number of steps to simulate.")
             .long("--steps")
             .short("-s")
             .validator( | val_str | {
                 match val_str.parse::<u128>() {
                     Ok(val) if val > 0 => Ok(()),
                     _ => Err(String::from("Specified port is not a positive integer value."))
                 }
             })
             .value_name("INT")
        )
        .settings(
            &[
                clap::AppSettings::ColoredHelp,
                clap::AppSettings::VersionlessSubcommands
            ]
        );
    argument_parser.get_matches()
}
