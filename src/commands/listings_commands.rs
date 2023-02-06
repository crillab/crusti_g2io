use super::logging_level_arg;
use anyhow::Result;
use crusti_app_helper::{App, AppSettings, ArgMatches, Command, SubCommand};
use crusti_g2io::{generators, linkers, NamedParam};

const CMD_NAME_GENERATORS: &str = "generators";

pub struct GeneratorsCommand;

impl GeneratorsCommand {
    pub(crate) fn new() -> Self {
        GeneratorsCommand
    }
}

impl<'a> Command<'a> for GeneratorsCommand {
    fn name(&self) -> &str {
        CMD_NAME_GENERATORS
    }

    fn clap_subcommand(&self) -> App<'a, 'a> {
        SubCommand::with_name(CMD_NAME_GENERATORS)
            .about("Lists the available graph generators")
            .setting(AppSettings::DisableVersion)
            .arg(logging_level_arg())
    }

    fn execute(&self, _arg_matches: &ArgMatches<'_>) -> Result<()> {
        print_listing(generators::iter_generator_factories());
        Ok(())
    }
}

const CMD_NAME_LINKERS: &str = "linkers";

pub struct LinkersCommand;

impl LinkersCommand {
    pub(crate) fn new() -> Self {
        LinkersCommand
    }
}

impl<'a> Command<'a> for LinkersCommand {
    fn name(&self) -> &str {
        CMD_NAME_LINKERS
    }

    fn clap_subcommand(&self) -> App<'a, 'a> {
        SubCommand::with_name(CMD_NAME_LINKERS)
            .about("Lists the available linkers")
            .setting(AppSettings::DisableVersion)
            .arg(logging_level_arg())
    }

    fn execute(&self, _arg_matches: &ArgMatches<'_>) -> Result<()> {
        print_listing(linkers::iter_linkers());
        Ok(())
    }
}

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
