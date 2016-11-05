extern crate clap;
extern crate combustion;

mod build;
mod error;
mod external;
mod init;

use clap::{App, SubCommand, AppSettings};

use build::{subcommand_build, subcommand_run};
use error::CliError;
use external::subcommand_external;
use init::subcommand_init;

pub fn run() {
    let app = App::new("Combustion CLI")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Command-line tool for interacting with a Combustion project.")
        .setting(AppSettings::AllowExternalSubcommands)
        .setting(AppSettings::VersionlessSubcommands)
        .subcommand(SubCommand::with_name("init")
            .about("Creates a new Combustion project in the current directory")
        )
        .subcommand(SubCommand::with_name("run")
            .about("Builds a Combustion project")
        )
        .subcommand(SubCommand::with_name("run")
            .about("Builds and executes a Combustion project")
        );
    let matches = app.get_matches();

    // Run the chosen subcommand
    let result = match matches.subcommand() {
        ("init",  Some(matches)) => subcommand_init(matches),
        ("build",  Some(matches)) => subcommand_build(matches),
        ("run",  Some(matches)) => subcommand_run(matches),
        ("", _) => subcommand_missing(),
        (name, _) => subcommand_external(name),
    };

    // Make sure the command succeeded
    if let Err(err) = result {
        println!("Error: {}", err.message);
    }
}

fn subcommand_missing() -> Result<(), CliError> {
    println!("A subcommand is required, run \"combustion help\" for help.");
    Ok(())
}
