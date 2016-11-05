use error::CliError;

pub fn subcommand_external(name: &str) -> Result<(), CliError> {
    println!("External subcommands are not yet implemented, run \"combustion-{}\" instead.", name);
    Ok(())
}
