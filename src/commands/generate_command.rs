use super::logging_level_arg;
use anyhow::{Context, Result};
use crusti_app_helper::{info, App, AppSettings, Arg, ArgMatches, Command, SubCommand};
use crusti_g2io::{
    generators::{self, BoxedGenerator},
    linkers::{self, BoxedLinker},
    InnerOuterGenerationStep, InnerOuterGenerator,
};
use petgraph::EdgeType;
use rand::SeedableRng;
use rand_pcg::Pcg32;
use std::{
    fs::File,
    io::{self, BufWriter, Write},
};

pub(crate) const ARG_INNER: &str = "INNER";
pub(crate) const ARG_OUTER: &str = "OUTER";
pub(crate) const ARG_LINKER: &str = "LINKER";
pub(crate) const ARG_GRAPH_FORMAT: &str = "GRAPH_FORMAT";
pub(crate) const ARG_EXPORT_TO_FILE: &str = "EXPORT_TO_FILE";
pub(crate) const ARG_SEED: &str = "SEED";

const CMD_NAME_DIRECTED: &str = "generate-directed";

pub struct GenerateDirectedCommand;

impl GenerateDirectedCommand {
    pub(crate) fn new() -> Self {
        GenerateDirectedCommand
    }
}

impl<'a> Command<'a> for GenerateDirectedCommand {
    fn name(&self) -> &str {
        CMD_NAME_DIRECTED
    }

    fn clap_subcommand(&self) -> App<'a, 'a> {
        SubCommand::with_name(CMD_NAME_DIRECTED)
            .about("Generates a directed graph")
            .setting(AppSettings::DisableVersion)
            .args(&args_for_generate())
    }

    fn execute(&self, arg_matches: &ArgMatches<'_>) -> Result<()> {
        execute_with(
            arg_matches,
            generators::directed_generator_factory_from_str,
            linkers::directed_linker_from_str,
        )
    }
}

const CMD_NAME_UNDIRECTED: &str = "generate-undirected";

pub struct GenerateUndirectedCommand;

impl GenerateUndirectedCommand {
    pub(crate) fn new() -> Self {
        GenerateUndirectedCommand
    }
}

impl<'a> Command<'a> for GenerateUndirectedCommand {
    fn name(&self) -> &str {
        CMD_NAME_UNDIRECTED
    }

    fn clap_subcommand(&self) -> App<'a, 'a> {
        SubCommand::with_name(CMD_NAME_UNDIRECTED)
            .about("Generates an undirected graph")
            .setting(AppSettings::DisableVersion)
            .args(&args_for_generate())
    }

    fn execute(&self, arg_matches: &ArgMatches<'_>) -> Result<()> {
        execute_with(
            arg_matches,
            generators::undirected_generator_factory_from_str,
            linkers::undirected_linker_from_str,
        )
    }
}

fn args_for_generate<'a>() -> Vec<Arg<'a, 'a>> {
    vec![
        Arg::with_name(ARG_INNER)
            .short("i")
            .long("inner")
            .empty_values(false)
            .multiple(false)
            .help("the kind of inner graphs")
            .required(true),
        Arg::with_name(ARG_OUTER)
            .short("o")
            .long("outer")
            .empty_values(false)
            .multiple(false)
            .help("the kind of outer graphs")
            .required(true),
        Arg::with_name(ARG_LINKER)
            .short("l")
            .long("linker")
            .empty_values(false)
            .multiple(false)
            .help("the linker used to connect inner graphs")
            .required(true),
        Arg::with_name(ARG_GRAPH_FORMAT)
            .short("f")
            .long("format")
            .multiple(false)
            .default_value("dot")
            .possible_values(&["dot", "graphml", "dimacs"])
            .help("the output format used for graphs"),
        Arg::with_name(ARG_EXPORT_TO_FILE)
            .short("x")
            .long("export")
            .empty_values(false)
            .multiple(false)
            .help("export the graph to the file instead of printing it"),
        Arg::with_name(ARG_SEED)
            .short("s")
            .long("seed")
            .empty_values(false)
            .multiple(false)
            .help("sets the seed for the random generator (64bits integer)"),
        logging_level_arg(),
    ]
}

fn execute_with<F, G, Ty>(
    arg_matches: &ArgMatches<'_>,
    generator_factory_from_str: F,
    linker_from_str: G,
) -> Result<()>
where
    F: Fn(&str) -> Result<BoxedGenerator<Ty, Pcg32>>,
    G: Fn(&str) -> Result<BoxedLinker<Ty, Pcg32>>,
    Ty: EdgeType + Send + Sync,
{
    let outer_generator = generator_factory_from_str(arg_matches.value_of(ARG_OUTER).unwrap())
        .context("while parsing the outer generator CLI argument")?;
    let inner_generator = generator_factory_from_str(arg_matches.value_of(ARG_INNER).unwrap())
        .context("while parsing the inner generator CLI argument")?;
    let linker = linker_from_str(arg_matches.value_of(ARG_LINKER).unwrap())
        .context("while parsing the linker CLI argument")?;
    let seed = match arg_matches.value_of(ARG_SEED) {
        Some(s) => s
            .parse::<u64>()
            .context("while reading the random seed from the command line")?,
        None => rand::random::<u64>(),
    };
    info!("random seed is {}", seed);
    let mut rng = rand_pcg::Pcg32::seed_from_u64(seed);
    let mut inner_outer_generator = InnerOuterGenerator::default();
    inner_outer_generator.add_generation_step_listener(Box::new(|step| match step {
        InnerOuterGenerationStep::OuterGeneration => {
            info!("beginning the outer graph generation")
        }
        InnerOuterGenerationStep::InnerGeneration => {
            info!("beginning the inner graphs generation")
        }
        InnerOuterGenerationStep::Linking => info!("beginning the linking"),
    }));
    let g = inner_outer_generator.new_inner_outer(
        outer_generator.as_ref(),
        inner_generator.as_ref(),
        linker.as_ref(),
        &mut rng,
    );
    info!(
        "generated a graph with {} nodes and {} edges",
        g.n_nodes(),
        g.n_edges()
    );
    let unbuffered_out: Box<dyn Write> = match arg_matches.value_of(ARG_EXPORT_TO_FILE) {
        None => Box::new(io::stdout()),
        Some(path) => Box::new(File::create(path).context("while creating the output file")?),
    };
    let mut out = BufWriter::new(unbuffered_out);
    match arg_matches.value_of(ARG_GRAPH_FORMAT).unwrap() {
        "dot" => writeln!(&mut out, "{}", g.to_dot_display()),
        "graphml" => writeln!(&mut out, "{}", g.to_graphml_display()),
        "dimacs" => writeln!(&mut out, "{}", g.to_dimacs_display()),
        _ => unreachable!(),
    }
    .context("while writing the graph")?;
    Ok(())
}
