use error::CliError;

pub fn subcommand_external(name: &str) -> Result<(), CliError> {
    Err(CliError {
        message: format!(
            "External subcommands are not yet implemented, run \"combustion-{}\" instead, or run \"combustion help\" for a list of subcommands.",
            name
        )
    })
}
