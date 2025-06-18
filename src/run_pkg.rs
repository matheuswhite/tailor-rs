use std::{path::PathBuf, process};

use crate::{
    build_pkg::BuildPkg,
    command::Command,
    mode::Mode,
    package::{Package, PackageType},
};

#[derive(Default)]
pub struct RunPkg {
    mode: Mode,
    path: PathBuf,
}

impl Command for RunPkg {
    fn parse_args(&mut self, args: &[String]) -> Option<()> {
        if args.is_empty() || args[0] != "run" {
            return None;
        }

        match args.len() {
            1 => {
                self.mode = Mode::Debug;
                self.path = std::env::current_dir().ok()?;

                Some(())
            }
            2 => {
                match args[1].as_str().try_into() {
                    Ok(mode) => {
                        self.mode = mode;
                        self.path = std::env::current_dir().ok()?;
                    }
                    Err(_) => {
                        self.mode = Mode::Debug;
                        self.path = PathBuf::from(&args[1]);
                    }
                }

                Some(())
            }
            3 => {
                let mode = match args[1].as_str().try_into() {
                    Ok(mode) => mode,
                    Err(_) => return None,
                };

                self.mode = mode;
                self.path = PathBuf::from(&args[2]);

                Some(())
            }
            _ => None,
        }
    }

    fn execute(&self) -> Result<(), String> {
        let mode_name = self.mode.to_string();
        let abs_path = self
            .path
            .canonicalize()
            .map_err(|_| "Failed to get absolute path")?;

        let pkg = Package::from_file(&abs_path.join("Tailor.toml"))
            .map_err(|_| "Failed to load package")?;

        match pkg.pkg_type() {
            PackageType::Library => {
                return Err("Cannot run a library package".to_string());
            }
            PackageType::Binary => {
                let mut build = BuildPkg::default();
                build
                    .parse_args(&[
                        "build".to_string(),
                        format!("--{}", mode_name),
                        abs_path.to_string_lossy().to_string(),
                    ])
                    .ok_or("Failed to parse build arguments".to_string())?;
                build.execute()?;

                process::Command::new(abs_path.join("build").join(mode_name).join(pkg.name()))
                    .status()
                    .map_err(|_| "Failed to execute binary")?;
            }
        }

        Ok(())
    }
}
