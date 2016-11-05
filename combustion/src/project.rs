use std::process::Command;
use std::path::{PathBuf};
use tar::Archive;

pub struct Project {
    _path: PathBuf,
    bin_path: PathBuf,
}

impl Project {
    /// Initializes a new Combustion project into a path.
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
        archive.unpack(&path).unwrap();

        // Open the newly created project
        Self::open(path)
    }

    /// Opens an existing Combustion project.
    pub fn open<P: Into<PathBuf>>(path: P) -> Result<Project, ProjectError> {
        let path = path.into();
        let mut bin_path = path.clone();
        bin_path.push("bin");

        Ok(Project {
            _path: path,
            bin_path: bin_path
        })
    }

    pub fn build(&self) -> Result<(), ProjectError> {
        self.build_assets()?;

        // TODO: Improve error handling
        Command::new("cargo")
            .arg("build")
            .current_dir(&self.bin_path)
            .spawn()
            .expect("cargo failed to start")
            .wait()
            .expect("cargo error'd out");

        Ok(())
    }

    pub fn run(&self) -> Result<(), ProjectError> {
        self.build_assets()?;

        // TODO: Improve error handling
        Command::new("cargo")
            .arg("run")
            .current_dir(&self.bin_path)
            .spawn()
            .expect("cargo failed to start")
            .wait()
            .expect("cargo error'd out");

        Ok(())
    }

    pub fn build_assets(&self) -> Result<(), ProjectError> {
        Ok(())
    }
}

pub enum ProjectError {
    InvalidTarget { message: String }
}
