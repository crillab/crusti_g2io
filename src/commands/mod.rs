mod generate_command;
pub use generate_command::GenerateCommand;

mod listings_commands;
pub use listings_commands::GeneratorsCommand;
pub use listings_commands::LinkersCommand;

use crusti_app_helper::Arg;

const APP_HELPER_LOGGING_LEVEL_ARG: &str = "APP_HELPER_LOGGING_LEVEL_ARG";

fn logging_level_arg<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name(APP_HELPER_LOGGING_LEVEL_ARG)
        .long("logging-level")
        .multiple(false)
        .default_value("info")
        .possible_values(&["trace", "debug", "info", "warn", "error", "off"])
        .help("set the minimal logging level")
}