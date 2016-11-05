use clap::{ArgMatches};
use combustion::project::{Project};
use error::CliError;

pub fn subcommand_build(_matches: &ArgMatches) -> Result<(), CliError> {
    let project = Project::open("./")?;
    project.build()?;
    Ok(())
}

pub fn subcommand_run(_matches: &ArgMatches) -> Result<(), CliError> {
    let project = Project::open("./")?;
    project.run()?;
    Ok(())
}
