mod error;

use std::process::Command;
use std::path::{PathBuf};
use tar::Archive;
use slog::Logger;

pub use self::error::ProjectError;

pub struct Project {
    binaries: Vec<PathBuf>,
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
        let bytes: &[u8] = include_bytes!("../../new-template.tar");
        let mut archive = Archive::new(bytes);
        archive.unpack(&path)?;

        // Open the newly created project
        Self::open(path)
    }

    /// Opens an existing Combustion project.
    pub fn open<P: Into<PathBuf>>(path: P) -> Result<Project, ProjectError> {
        let path = path.into();

        // Scan the binaries folder for binaries
        let mut bin_path = path.clone();
        bin_path.push("binaries");
        let mut binaries = Vec::new();
        for item in bin_path.read_dir()? {
            binaries.push(item?.path());
        }

        Ok(Project {
            binaries: binaries
        })
    }

    pub fn build(&self, log: &Logger) -> Result<(), ProjectError> {
        // First build the assets if we need to
        self.build_assets()?;

        // Then, build all binaries
        for binary in &self.binaries {
            // TODO: Change to use logging
            info!(log, "Building \"{}\"", binary.file_name().unwrap().to_str().unwrap());
            self.cargo_command(&binary, "build")?;
        }

        Ok(())
    }

    pub fn run(&self, log: &Logger) -> Result<(), ProjectError> {
        // First run the build command
        self.build(log)?;

        // Find the game binary
        let mut main_binary = None;
        for binary in &self.binaries {
            if binary.file_name().unwrap() == "game" {
                main_binary = Some(binary);
                break;
            }
        }

        // Make sure we found it
        if main_binary.is_none() {
            return Err(ProjectError::InvalidProject {
                message: "Couldn't find \"game\" binary in project".into()
            });
        }

        // Then run that binary TODO: Change to use logging
        info!(log, "Running \"game\" binary");
        self.cargo_command(&main_binary.unwrap(), "build")?;

        Ok(())
    }

    pub fn build_assets(&self) -> Result<(), ProjectError> {
        Ok(())
    }

    fn cargo_command(&self, path: &PathBuf, command: &str) -> Result<(), ProjectError> {
        // TODO: Improve error handling
        Command::new("cargo")
            .arg(command)
            .current_dir(path)
            .spawn()
            .expect("cargo failed to start")
            .wait()
            .expect("cargo error'd out");

        Ok(())
    }
}
