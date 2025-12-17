use crate::{
    absolute_path::AbsolutePath,
    command::Command,
    external_tool::{cmake::CMake, custom_tool::CustomTool, registry::Registry},
    fmt::success,
    manifest::Manifest,
    mode::Mode,
    package::Package,
};
use std::path::PathBuf;

#[derive(Default)]
pub struct BuildPkg {
    path: AbsolutePath,
    mode: Mode,
    registry: Registry,
}

impl Command for BuildPkg {
    fn parse_args(&mut self, args: &[String]) -> Option<()>
    where
        Self: Sized,
    {
        if args.is_empty() || args[0] != "build" {
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
            .map_err(|_| "fail to read Tailor.toml".to_string())?;
        let manifest = Manifest::from_file(&manifest_content, &self.path)?;
        let pkg = Package::load_from_manifest(manifest, &self.registry)?;

        let sources = pkg.sources();
        let includes = pkg.includes();
        let pkg_type = pkg.pkg_type()?;
        let base_path = self.path.inner().join("build");
        let defines = pkg
            .options()
            .into_iter()
            .map(|def| def.to_define())
            .collect();

        if let Some(tool) = pkg.tool() {
            CustomTool::build(
                self.mode,
                &self.path,
                tool,
                sources,
                includes,
                &self.registry,
            )
        } else {
            let path = match self.mode {
                Mode::Debug => base_path.join("debug"),
                Mode::Release => base_path.join("release"),
            };
            std::fs::create_dir_all(&path)
                .map_err(|e| format!("fail to create build directory: {}", e))?;

            // TODO: Regen if Tailor.toml changes
            CMake::create_cmake_lists(
                pkg_type,
                sources.clone(),
                includes.clone(),
                path.try_into()?,
            )?;

            println!(
                "{} `{}` in {} mode",
                success("Building"),
                pkg.name(),
                mode_name
            );

            CMake::build(self.mode, &self.path, defines)
        }
    }
}
