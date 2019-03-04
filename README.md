# grav

A (work-in-progress) tui-based nbody simulation program written in rust.

----
## Usage

`grav` is invoked with one of a few different "subcommands" on the command line: `help,`, ``load`, or `new`. The `new` subcommand starts a new simulation with either the default parameters and starting condition, or with the specification defined in a simulation configuration YAML file:

```bash
$ grav new --config 'example.yaml'
```

The `load` subcommand allows the user to load a previously-saved simulation file. We'll cover the subcommands in more detail below.

### Common CLI Arguments

The following table describes the program's common CLI arguments:

| Argument(s)         | Description                                                             |
|---------------------|-------------------------------------------------------------------------|
| `-h`, `--help`      | Displays help and usage information.                                    |
| `-f`, `--log-file`  | Specifies the log file to write to.                                     |
| `-l`, `--log-level` | Specifies the log level of the program.                                 |
| `-M`, `--log-mode`  | Specifies whether to `append` to or `overwrite` the specified log file. |
| `-V`, `--version`   | Displays version information.                                           |

Each of the above options has the following set of corresponding value types and default values:

| Argument(s)         | Value Type / Possible Values                                | Default Value |
|---------------------|-------------------------------------------------------------|---------------|
| `-f`, `--log-file`  | File Path                                                   | `grav.log`    |
| `-l`, `--log-level` | `disabled`, `error`, `warning`, `info`, `debug`, or `trace` | `info`        |
| `-M`, `--log-mode`  | `append` or `overwrite`                                     | `append`      |

### Environment Variables

The default behavior of the `grav` program may also be configured via environment variables. Each environment variable has a corresponding command-line argument, as described in the table below:

| Environment Variable | Correpsonding CLI Argument |
|----------------------|----------------------------|
| `GRAV_LOG_FILE`      | `--log-file`               |
| `GRAV_LOG_LEVEL`     | `--log-level`              |
| `GRAV_LOG_MODE`      | `--log-mode`               |
