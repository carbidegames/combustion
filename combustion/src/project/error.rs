pub enum ProjectError {
    InvalidTarget { message: String },
    InvalidProject { message: String },
    IoError { inner: ::std::io::Error },
}

impl From<::std::io::Error> for ProjectError {
    fn from(error: ::std::io::Error) -> Self {
        ProjectError::IoError {
            inner: error
        }
    }
}
