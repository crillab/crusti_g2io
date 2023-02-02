use super::logging_level_arg;
use crate::{generators::FACTORIES_THREAD_RNG, linkers::LINKERS, utils::Named};
use anyhow::Result;
use crusti_app_helper::{App, AppSettings, ArgMatches, Command, SubCommand};

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
        print_listing(FACTORIES_THREAD_RNG.as_slice());
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
        print_listing(LINKERS.as_slice());
        Ok(())
    }
}

fn print_listing<S, T>(collection: &[Box<S>])
where
    S: Named<T> + Sync + ?Sized,
{
    let name_display_size = collection
        .iter()
        .map(|f| f.name().len())
        .max()
        .unwrap_or_default()
        + 4;
    collection.iter().for_each(|f| {
        let first_description_line = if f.description().is_empty() {
            ""
        } else {
            f.description()[0]
        };
        println!("{:name_display_size$}{}", f.name(), first_description_line);
        f.description().iter().skip(1).for_each(|line| {
            println!("{:name_display_size$}{}", "", line);
        });
        println!()
    });
}
