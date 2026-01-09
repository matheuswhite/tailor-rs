use crate::{
    absolute_path::AbsolutePath,
    command::{Command, build_pkg::BuildPkg},
    external_tool::{cmake::CMake, registry::Registry},
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
    fn parse_args(&mut self, args: &[String]) -> Option<()> {
        if args.is_empty() || args[0] != "run" {
            return None;
        }

        match args.len() {
            1 => {
                self.mode = Mode::Debug;
                self.path = std::env::current_dir().ok()?.try_into().ok()?;

                Some(())
            }
            2 => {
                match args[1].as_str().try_into() {
                    Ok(mode) => {
                        self.mode = mode;
                        self.path = std::env::current_dir().ok()?.try_into().ok()?;
                    }
                    Err(_) => {
                        self.mode = Mode::Debug;
                        self.path = PathBuf::from(&args[1]).try_into().ok()?;
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
                self.path = PathBuf::from(&args[2]).try_into().ok()?;

                Some(())
            }
            _ => None,
        }
    }

    fn execute(&self) -> Result<(), String> {
        let mode_name = self.mode.to_string();

        let manifest_content = std::fs::read_to_string(self.path.inner().join("Tailor.toml"))
            .map_err(|_| "fail to read Tailor.toml")?;
        let manifest = Manifest::from_file(&manifest_content, &self.path)?;
        let pkg = Package::load_from_manifest(manifest, &self.registry)?;

        let pkg_type = pkg.pkg_type();
        let pkg_name = pkg.name();

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
                    .ok_or("Failed to parse build arguments".to_string())?;
                build.execute()?;

                println!(
                    "{} `{}` in {} mode",
                    success("Running"),
                    pkg_name,
                    mode_name
                );

                CMake::run(self.mode, &self.path)
            }
        }
    }
}
