use crate::{
    absolute_path::AbsolutePath,
    command::Command,
    external_tool::{compiler::Compiler, registry::Registry},
    fmt::success,
    manifest::Manifest,
    mode::Mode,
    package::Package,
};
use std::{path::PathBuf, time::Instant};

#[derive(Default)]
pub struct BuildPkg {
    path: AbsolutePath,
    mode: Mode,
    registry: Registry,
}

impl Command for BuildPkg {
    fn parse_args(&mut self, args: &[String]) -> Result<bool, String>
    where
        Self: Sized,
    {
        if args.is_empty() || args[0] != "build" {
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
                        self.path = PathBuf::from(&args[1]).try_into()?;
                    }
                }

                Ok(true)
            }
            3 => {
                let mode = match args[1].as_str().try_into() {
                    Ok(mode) => mode,
                    Err(_) => return Err("Invalid mode".to_string()),
                };

                self.mode = mode;
                self.path = PathBuf::from(&args[2]).try_into()?;

                Ok(true)
            }
            _ => Err("Too many arguments for build command".to_string()),
        }
    }

    fn execute(&self) -> Result<(), String> {
        let start = Instant::now();
        let manifest_content = std::fs::read_to_string(self.path.inner().join("Tailor.toml"))
            .map_err(|_| "fail to read Tailor.toml".to_string())?;
        let manifest = Manifest::from_file(&manifest_content, &self.path)?;
        let pkg = Package::load_from_manifest(manifest, &self.registry)?;

        let manifest = pkg.manifest();
        let pkg_type = manifest.pkg_type();
        let base_path = self.path.inner().join("build");
        let defines = pkg
            .options()
            .into_iter()
            .map(|def| def.to_define())
            .collect();

        let path = match self.mode {
            Mode::Debug => base_path.join("debug"),
            Mode::Release => base_path.join("release"),
        };
        std::fs::create_dir_all(&path)
            .map_err(|e| format!("fail to create build directory: {}", e))?;

        let compiler = Compiler::new(manifest.compiler(), manifest.full_name());

        compiler.build(self.mode, &path, pkg, pkg_type, defines)?;

        println!(
            "{} `{}` profile target in {:.2}s",
            success("Finished"),
            self.mode,
            start.elapsed().as_secs_f32()
        );

        Ok(())
    }
}
