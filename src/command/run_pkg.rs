use crate::{
    absolute_path::AbsolutePath,
    command::{Command, build_pkg::BuildPkg},
    external_tool::registry::Registry,
    fmt::success,
    manifest::{Manifest, package_type::PackageType},
    mode::Mode,
    package::Package,
};
use std::path::PathBuf;

#[derive(Default)]
pub struct RunPkg {
    mode: Mode,
    path: AbsolutePath,
    registry: Registry,
}

impl Command for RunPkg {
    fn parse_args(&mut self, args: &[String]) -> Result<bool, String> {
        if args.is_empty() || args[0] != "run" {
            return Ok(false);
        }

        match args.len() {
            1 => {
                self.mode = Mode::Debug;
                self.path = std::env::current_dir()
                    .map_err(|err| err.to_string())?
                    .try_into()?;

                Ok(true)
            }
            2 => {
                match args[1].as_str().try_into() {
                    Ok(mode) => {
                        self.mode = mode;
                        self.path = std::env::current_dir()
                            .map_err(|err| err.to_string())?
                            .try_into()?;
                    }
                    Err(_) => {
                        self.mode = Mode::Debug;
                        self.path = PathBuf::from(&args[1])
                            .try_into()
                            .map_err(|_| "invalid path".to_string())?;
                    }
                }

                Ok(true)
            }
            3 => {
                let mode = match args[1].as_str().try_into() {
                    Ok(mode) => mode,
                    Err(_) => return Err("invalid mode".to_string()),
                };

                self.mode = mode;
                self.path = PathBuf::from(&args[2])
                    .try_into()
                    .map_err(|_| "invalid path".to_string())?;

                Ok(true)
            }
            _ => Err("invalid arguments".to_string()),
        }
    }

    fn execute(&self) -> Result<(), String> {
        let mode_name = self.mode.to_string();

        let manifest_content = std::fs::read_to_string(self.path.inner().join("Tailor.toml"))
            .map_err(|_| "fail to read Tailor.toml")?;
        let manifest = Manifest::from_file(&manifest_content, &self.path)?;
        let pkg = Package::load_from_manifest(manifest, &self.registry)?;

        let pkg_type = pkg.manifest().pkg_type();
        let pkg_name = pkg.manifest().full_name();

        match pkg_type {
            PackageType::Library => Err("It's not possible run a library package".to_string()),
            PackageType::Binary => {
                let mut build = BuildPkg::default();
                build
                    .parse_args(&[
                        "build".to_string(),
                        format!("--{}", mode_name),
                        self.path.inner().to_string_lossy().to_string(),
                    ])
                    .map_err(|_| "Failed to parse build arguments".to_string())?;
                build.execute()?;

                let executable_path = self
                    .path
                    .inner()
                    .join("build")
                    .join(mode_name)
                    .join(&pkg_name);
                println!(
                    "{} `{}`",
                    success("Running"),
                    executable_path.to_string_lossy()
                );
                let status = std::process::Command::new(executable_path)
                    .status()
                    .map_err(|e| format!("failed to execute the package: {}", e))?;
                if !status.success() {
                    return Err("execution failed".to_string());
                }

                Ok(())
            }
        }
    }
}
