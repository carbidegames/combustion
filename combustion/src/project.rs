use std::path::PathBuf;
use tar::Archive;

pub struct Project {
}

impl Project {
    pub fn init<P: Into<PathBuf>>(path: P) -> Result<Project, ProjectError> {
        let path = path.into();

        // Make sure there's not already a project here
        let mut toml_path = path.clone();
        toml_path.push("Combustion.toml");
        if toml_path.exists() {
            return Err(ProjectError::InvalidTarget {
                message: format!("A project already exists at target path \"{}\"", path.display())
            });
        }

        // Now that we know we can initialize to here, do it
        let bytes: &[u8] = include_bytes!("../new-template.tar");
        let mut archive = Archive::new(bytes);
        archive.unpack(path).unwrap();

        Ok(Project {
        })
    }
}

pub enum ProjectError {
    InvalidTarget { message: String }
}
