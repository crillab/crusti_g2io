mod display;
mod generators;
mod graph;

use anyhow::{Context, Result};
use crusti_app_helper::{AppHelper, AppSettings, Arg, ArgMatches, Command, SubCommand};
use graph::Graph;

pub(crate) fn create_app_helper() -> AppHelper<'static> {
    let app_name = option_env!("CARGO_PKG_NAME").unwrap_or("unknown app name");
    let app_version = option_env!("CARGO_PKG_VERSION").unwrap_or("unknown version");
    let authors = option_env!("CARGO_PKG_AUTHORS").unwrap_or("unknown authors");
    let mut app = AppHelper::new(
        app_name,
        app_version,
        authors,
        "crusti_g2io, a Graph Generator following an Inner/Outer pattern.",
    );
    let commands: Vec<Box<dyn Command>> = vec![Box::new(GenerateCommand::new())];
    for c in commands {
        app.add_command(c);
    }
    app
}

const CMD_NAME: &str = "generate";

pub(crate) const ARG_INNER: &str = "INNER";
pub(crate) const ARG_OUTER: &str = "OUTER";
const APP_HELPER_LOGGING_LEVEL_ARG: &str = "APP_HELPER_LOGGING_LEVEL_ARG";

pub(crate) struct GenerateCommand;

impl GenerateCommand {
    pub(crate) fn new() -> Self {
        GenerateCommand
    }
}

impl<'a> Command<'a> for GenerateCommand {
    fn name(&self) -> &str {
        CMD_NAME
    }

    fn clap_subcommand(&self) -> crusti_app_helper::App<'a, 'a> {
        SubCommand::with_name(CMD_NAME)
            .about("Generates a graph")
            .setting(AppSettings::DisableVersion)
            .arg(
                Arg::with_name(ARG_INNER)
                    .short("i")
                    .long("inner")
                    .empty_values(false)
                    .multiple(false)
                    .help("the kind of inner graphs")
                    .required(true),
            )
            .arg(
                Arg::with_name(ARG_OUTER)
                    .short("o")
                    .long("outer")
                    .empty_values(false)
                    .multiple(false)
                    .help("the kind of outer graphs")
                    .required(true),
            )
            .arg(
                Arg::with_name(APP_HELPER_LOGGING_LEVEL_ARG)
                    .long("logging-level")
                    .multiple(false)
                    .default_value("info")
                    .possible_values(&["trace", "debug", "info", "warn", "error", "off"])
                    .help("set the minimal logging level"),
            )
    }

    fn execute(&self, arg_matches: &ArgMatches<'_>) -> Result<()> {
        let outer_generator =
            generators::generator_factory_from_str(arg_matches.value_of(ARG_OUTER).unwrap())
                .context("while parsing the outer generator CLI argument")?;
        let inner_generator =
            generators::generator_factory_from_str(arg_matches.value_of(ARG_INNER).unwrap())
                .context("while parsing the inner generator CLI argument")?;
        let mut rng = rand::thread_rng();
        let g = Graph::new_inner_outer(
            outer_generator.as_ref(),
            inner_generator.as_ref(),
            |g1, _g2| vec![(g1.n_nodes() - 1, 0)],
            &mut rng,
        );
        println!("{}", g.to_graphml_display());
        Ok(())
    }
}

fn main() {
    let app = create_app_helper();
    app.launch_app();
}
