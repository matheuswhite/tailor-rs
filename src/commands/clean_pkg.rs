use std::{fs::remove_dir_all, io::ErrorKind, path::PathBuf};

#[allow(unused_imports)]
use crate::{
    cmake,
    commands::command::Command,
    dependency_manager::resolve_dependencies,
    fmt::{info, success},
    mode::Mode,
    package::{Package, PackageType},
};

#[derive(Default)]
pub struct CleanPkg {
    path: PathBuf,
    mode: Mode,
}

impl Command for CleanPkg {
    fn parse_args(&mut self, args: &[String]) -> Option<()>
    where
        Self: Sized,
    {
        if args.is_empty() || args[0] != "clean" {
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
            _ => return None,
        };

        self.path.push("build");
        self.path.push(self.mode.to_string());

        Some(())
    }

    fn execute(&self) -> Result<(), String> {
        match self.path.canonicalize() {
            Ok(abs_path) => {
                remove_dir_all(abs_path.clone())
                    .map_err(|err| format!("Failed to remove {:?} [{}]", abs_path, err))?;
            }
            Err(err) => {
                if err.kind() != ErrorKind::NotFound {
                    return Err("Fail".to_string());
                }
            }
        }
        Ok(())
    }
}
