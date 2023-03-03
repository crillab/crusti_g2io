use super::logging_level_arg;
use anyhow::Result;
use crusti_app_helper::{App, AppSettings, ArgMatches, Command, SubCommand};
use crusti_g2io::{display, generators, linkers, NamedParam};

macro_rules! listing_cmd {
    ($cmd_ident:ident, $cmd_name:expr, $cmd_description:expr, $listing_fn:expr) => {
        pub struct $cmd_ident;

        impl $cmd_ident {
            pub(crate) fn new() -> Self {
                Self
            }
        }

        impl<'a> Command<'a> for $cmd_ident {
            fn name(&self) -> &str {
                $cmd_name
            }

            fn clap_subcommand(&self) -> App<'a, 'a> {
                SubCommand::with_name($cmd_name)
                    .about($cmd_description)
                    .setting(AppSettings::DisableVersion)
                    .arg(logging_level_arg())
            }

            fn execute(&self, _arg_matches: &ArgMatches<'_>) -> Result<()> {
                print_listing($listing_fn);
                Ok(())
            }
        }
    };
}

listing_cmd!(
    GeneratorsDirectedCommand,
    "generators-directed",
    "Lists the available directed graph generators",
    generators::iter_directed_generator_factories()
);

listing_cmd!(
    GeneratorsUndirectedCommand,
    "generators-undirected",
    "Lists the available undirected graph generators",
    generators::iter_undirected_generator_factories()
);

listing_cmd!(
    LinkersDirectedCommand,
    "linkers-directed",
    "Lists the available linkers for directed graphs",
    linkers::iter_directed_linkers()
);

listing_cmd!(
    LinkersUndirectedCommand,
    "linkers-undirected",
    "Lists the available linkers for undirected graphs",
    linkers::iter_undirected_linkers()
);

listing_cmd!(
    DisplayEnginesUndirectedCommand,
    "display-engines-undirected",
    "Lists the available display engines for undirected graphs",
    display::iter_undirected_display_engines()
);

listing_cmd!(
    DisplayEnginesDirectedCommand,
    "display-engines-directed",
    "Lists the available display engines for directed graphs",
    display::iter_directed_display_engines()
);

fn print_listing<I, S, T>(collection: I)
where
    I: Iterator<Item = &'static S>,
    S: NamedParam<T> + Sync + ?Sized + 'static,
{
    let listing: Vec<(&str, Vec<&str>)> = collection.map(|f| (f.name(), f.description())).collect();
    let name_display_size = listing.iter().map(|l| l.0.len()).max().unwrap_or_default() + 4;
    listing.iter().for_each(|l| {
        let first_description_line = if l.1.is_empty() { "" } else { l.1[0] };
        println!("{:name_display_size$}{}", l.0, first_description_line);
        l.1.iter().skip(1).for_each(|line| {
            println!("{:name_display_size$}{}", "", line);
        });
        println!()
    });
}
