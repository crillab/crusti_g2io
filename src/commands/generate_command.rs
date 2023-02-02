use std::{io::{self, Write}, fs::File};
use super::logging_level_arg;
use crate::{generators, graph::Graph, linkers};
use anyhow::{Context, Result};
use crusti_app_helper::{App, AppSettings, Arg, ArgMatches, Command, SubCommand, info};

const CMD_NAME: &str = "generate";

pub(crate) const ARG_INNER: &str = "INNER";
pub(crate) const ARG_OUTER: &str = "OUTER";
pub(crate) const ARG_LINKER: &str = "LINKER";
pub(crate) const ARG_GRAPH_FORMAT: &str = "GRAPH_FORMAT";
pub(crate) const ARG_SAVE_TO_FILE: &str = "SAVE_TO_FILE";

pub struct GenerateCommand;

impl GenerateCommand {
    pub(crate) fn new() -> Self {
        GenerateCommand
    }
}

impl<'a> Command<'a> for GenerateCommand {
    fn name(&self) -> &str {
        CMD_NAME
    }

    fn clap_subcommand(&self) -> App<'a, 'a> {
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
                Arg::with_name(ARG_LINKER)
                    .short("l")
                    .long("linker")
                    .empty_values(false)
                    .multiple(false)
                    .help("the linker used to connect inner graphs")
                    .required(true),
            )
            .arg(
                Arg::with_name(ARG_GRAPH_FORMAT)
                    .short("f")
                    .long("format")
                    .multiple(false)
                    .default_value("dot")
                    .possible_values(&["dot", "graphml"])
                    .help("the output format used for graphs"),
            )
            .arg(
                Arg::with_name(ARG_SAVE_TO_FILE)
                    .short("s")
                    .long("save")
                    .empty_values(false)
                    .multiple(false)
                    .help("save the graph to the file instead of printing it")
            )
            .arg(logging_level_arg())
    }

    fn execute(&self, arg_matches: &ArgMatches<'_>) -> Result<()> {
        let outer_generator =
            generators::generator_factory_from_str(arg_matches.value_of(ARG_OUTER).unwrap())
                .context("while parsing the outer generator CLI argument")?;
        let inner_generator =
            generators::generator_factory_from_str(arg_matches.value_of(ARG_INNER).unwrap())
                .context("while parsing the inner generator CLI argument")?;
        let linker = linkers::linker_from_str(arg_matches.value_of(ARG_LINKER).unwrap())
            .context("while parsing the linker CLI argument")?;
        let mut rng = rand::thread_rng();
        let g = Graph::new_inner_outer(
            outer_generator.as_ref(),
            inner_generator.as_ref(),
            linker.as_ref(),
            &mut rng,
        );
        info!("generated a graph with {} nodes and {} edges", g.n_nodes(), g.n_edges());
        let mut out: Box<dyn Write> = match arg_matches.value_of(ARG_SAVE_TO_FILE) {
            None => Box::new(io::stdout()),
            Some(path) => Box::new(File::create(path).context("while creating the output file")?),
        };
        match arg_matches.value_of(ARG_GRAPH_FORMAT).unwrap() {
            "dot" => writeln!(&mut out, "{}", g.to_dot_display()),
            "graphml" => writeln!(&mut out, "{}", g.to_graphml_display()),
            _ => unreachable!(),
        }.context("while writing the graph")?;
        Ok(())
    }
}
