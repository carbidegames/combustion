use clap::{ArgMatches};
use slog::Logger;
use combustion::project::{Project};
use error::CliError;

pub fn subcommand_build(_matches: &ArgMatches, log: &Logger) -> Result<(), CliError> {
    let project = Project::open("./")?;
    project.build(log)?;
    Ok(())
}

pub fn subcommand_run(_matches: &ArgMatches, log: &Logger) -> Result<(), CliError> {
    let project = Project::open("./")?;
    project.run(log)?;
    Ok(())
}
