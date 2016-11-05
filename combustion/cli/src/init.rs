use clap::{ArgMatches};
use combustion::project::{Project};
use error::CliError;

pub fn subcommand_init(_matches: &ArgMatches) -> Result<(), CliError> {
    let _project = Project::init("./")?;
    Ok(())
}
