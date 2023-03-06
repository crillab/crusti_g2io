use super::logging_level_arg;
use anyhow::{Context, Result};
use crusti_app_helper::{info, App, AppSettings, Arg, ArgMatches, Command, SubCommand};
use crusti_g2io::{
    display::{self, BoxedDisplay},
    generators::{self, BoxedGenerator},
    linkers::{self, BoxedLinker},
    Graph, InnerOuterGenerationStep, InnerOuterGenerator,
};
use petgraph::EdgeType;
use rand::SeedableRng;
use rand_pcg::Pcg32;
use std::{
    fmt::Display,
    fs::{self, File},
    io::{self, BufWriter, Write},
    path::PathBuf,
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
            display::directed_display_engine_from_str,
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
            display::undirected_display_engine_from_str,
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
            .empty_values(false)
            .multiple(false)
            .help("the output format used for graphs")
            .required(true),
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

fn execute_with<F, G, H, Ty>(
    arg_matches: &ArgMatches<'_>,
    generator_factory_from_str: F,
    linker_from_str: G,
    display_from_str: H,
) -> Result<()>
where
    F: Fn(&str) -> Result<BoxedGenerator<Ty, Pcg32>>,
    G: Fn(&str) -> Result<BoxedLinker<Ty, Pcg32>>,
    H: Fn(&str) -> Result<BoxedDisplay<Ty>>,
    Ty: EdgeType + Send + Sync,
{
    let outer_generator = generator_factory_from_str(arg_matches.value_of(ARG_OUTER).unwrap())
        .context("while parsing the outer generator CLI argument")?;
    let inner_generator = generator_factory_from_str(arg_matches.value_of(ARG_INNER).unwrap())
        .context("while parsing the inner generator CLI argument")?;
    let linker = linker_from_str(arg_matches.value_of(ARG_LINKER).unwrap())
        .context("while parsing the linker CLI argument")?;
    let display_engine = display_from_str(arg_matches.value_of(ARG_GRAPH_FORMAT).unwrap())
        .context("while parsing the display engine CLI argument")?;
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
    let (str_out, unbuffered_out): (String, Box<dyn Write>) =
        match arg_matches.value_of(ARG_EXPORT_TO_FILE) {
            None => ("standard output".to_string(), Box::new(io::stdout())),
            Some(path) => {
                let str_path = fs::canonicalize(&PathBuf::from(path))
                    .with_context(|| format!(r#"while opening file "{}""#, path))?;
                (
                    format!("{:?}", str_path),
                    Box::new(File::create(path).context("while creating the output file")?),
                )
            }
        };
    info!("writing graph to {}", str_out);
    let mut out = BufWriter::new(unbuffered_out);
    writeln!(
        &mut out,
        "{}",
        DisplayEngineWrapper {
            display_engine,
            graph: &g,
        }
    )
    .context("while writing the graph")?;
    Ok(())
}

type GraphDisplayFn<Ty> = dyn Fn(&mut std::fmt::Formatter, &Graph<Ty>) -> std::fmt::Result;

struct DisplayEngineWrapper<'a, Ty>
where
    Ty: EdgeType,
{
    display_engine: Box<GraphDisplayFn<Ty>>,
    graph: &'a Graph<Ty>,
}

impl<Ty> Display for DisplayEngineWrapper<'_, Ty>
where
    Ty: EdgeType,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self.display_engine)(f, self.graph)
    }
}
