// {% include 'doc.template' %}
// {% do require('this.args', 'this.subcommands') %}

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
        // {% for arg in this.args %}
        .arg(clap::Arg::with_name("{{ arg.name }}")
             // {% if arg.default is defined %}
             .default_value("{{ arg.default }}")
             // {% endif %}
             // {% if arg.env is defined %}
             .env("{{ arg.env }}")
             // {% endif %}
             .help("{{ arg.help }}")
             // {% if arg.long is defined %}
             .long("{{ arg.long }}")
             // {% endif %}
             // {% if arg.possible_values is defined %}
             .possible_values(&[
                 // {% for val in arg.possible_values %}
                 "{{ val }}",
                 // {% endfor %}
             ])
             // {% endif %}
             // {% if arg.short is defined %}
             .short("{{ arg.short }}")
             // {% endif %}
             // {% if arg.value_name is defined %}
             .value_name("{{ arg.value_name }}")
             // {% endif %}
        )
        // {% endfor %}
        // {% for sub in this.subcommands %}
        .subcommand(clap::SubCommand::with_name("{{ sub.name }}")
                    .about("{{ sub.help }}")
                    .help_message("Displays help and usage information.")
                    // {% if sub.args is defined %}
                    // {% for arg in sub.args %}
                    .arg(clap::Arg::with_name("{{ arg.name }}")
                         // {% if arg.default is defined %}
                         .default_value("{{ arg.default }}")
                         // {% endif %}
                         // {% if arg.env is defined %}
                         .env("{{ arg.env }}")
                         // {% endif %}
                         .help("{{ arg.help }}")
                         // {% if arg.long is defined %}
                         .long("{{ arg.long }}")
                         // {% endif %}
                         // {% if arg.possible_values is defined %}
                         .possible_values(&[
                             // {% for val in arg.possible_values %}
                             "{{ val }}",
                             // {% endfor %}
                         ])
                         // {% endif %}
                         // {% if arg.required is defined and arg.required %}
                         .required(true)
                         // {% endif %}
                         // {% if arg.short is defined %}
                         .short("{{ arg.short }}")
                         // {% endif %}
                         // {% if arg.value_name is defined %}
                         .value_name("{{ arg.value_name }}")
                         // {% endif %}
                    )
                    // {% endfor %}
                    // {% endif %}
                    .settings(
                        &[
                            clap::AppSettings::ColoredHelp,
                            clap::AppSettings::VersionlessSubcommands
                        ]
                    )
        )
        // {% endfor %}
        .settings(
            &[
                clap::AppSettings::ColoredHelp,
                clap::AppSettings::SubcommandRequiredElseHelp,
                clap::AppSettings::VersionlessSubcommands
            ]
        );
    argument_parser.get_matches()
}
