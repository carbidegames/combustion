extern crate clap;
extern crate combustion;

mod error;
mod external;
mod init;

use clap::{Arg, App, SubCommand, AppSettings, ArgMatches};

use error::CliError;
use external::subcommand_external;
use init::subcommand_init;

pub fn run() {
    let app = App::new("Combustion CLI")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Command-line tool for interacting with a Combustion project.")
        .setting(AppSettings::AllowExternalSubcommands)
        .setting(AppSettings::VersionlessSubcommands)
        .subcommand(SubCommand::with_name("hello")
            .about("Says hello")
            .arg(Arg::with_name("message"))
        )
        .subcommand(SubCommand::with_name("init")
            .about("Creates a new Combustion project in the current directory")
            .arg(Arg::with_name("message"))
        );
    let matches = app.get_matches();

    // Run the chosen subcommand
    let result = match matches.subcommand() {
        ("hello",  Some(matches)) => subcommand_hello(matches),
        ("init",  Some(matches)) => subcommand_init(matches),
        ("", _) => subcommand_missing(),
        (name, _) => subcommand_external(name),
    };

    // Make sure the command succeeded
    if let Err(err) = result {
        println!("Error: {}", err.message);
    }
}

fn subcommand_hello(matches: &ArgMatches) -> Result<(), CliError> {
    let msg = if let Some(v) = matches.value_of("message") {
        v
    } else {
        "Hello, world!"
    };

    println!("{}", msg);
    Ok(())
}

fn subcommand_missing() -> Result<(), CliError> {
    println!("A subcommand is required, run \"combustion help\" for help.");
    Ok(())
}
