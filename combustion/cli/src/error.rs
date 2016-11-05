use combustion::project::{ProjectError};

pub struct CliError {
    pub message: String
}

impl From<ProjectError> for CliError {
    fn from(error: ProjectError) -> Self {
        let message = match error {
            ProjectError::InvalidTarget { message } => message,
            ProjectError::InvalidProject { message } => message,
            ProjectError::IoError { inner } => format!("{}", inner),
        };

        CliError {
            message: message
        }
    }
}
