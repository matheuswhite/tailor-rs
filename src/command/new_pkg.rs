use crate::{command::Command, manifest::package_type::PackageType};
use std::path::PathBuf;

#[derive(Default)]
pub struct NewPkg {
    path: PathBuf,
    name: String,
    pkg_type: PackageType,
}

impl Command for NewPkg {
    fn help(&self) -> String {
        String::from(
            "Usage: tailor new [--bin|--lib] <path>\n\n\
            Create a new Tailor package at the specified path.\n\n\
            Options:\n\
            \t--bin\tCreate a binary (application) package (default)\n\
            \t--lib\tCreate a library package",
        )
    }

    fn parse_args(&mut self, args: &[String]) -> Result<bool, String>
    where
        Self: Sized,
    {
        if args.is_empty() || args[0] != "new" {
            return Ok(false);
        }

        match args.len() {
            1 => Err("Too few arguments".to_string()),
            2 => {
                self.path = PathBuf::from(&args[1]);
                self.name = self
                    .path
                    .file_name()
                    .and_then(|s| s.to_str())
                    .map(String::from)
                    .unwrap_or_default();

                Ok(true)
            }
            3 => {
                match args[1].as_str() {
                    "--bin" => self.pkg_type = PackageType::Binary,
                    "--lib" => self.pkg_type = PackageType::Library,
                    _ => return Err(format!("unknown flag: {}", args[1])),
                }

                self.path = PathBuf::from(&args[2]);
                self.name = self
                    .path
                    .file_name()
                    .and_then(|s| s.to_str())
                    .map(String::from)
                    .unwrap_or_default();

                Ok(true)
            }
            _ => Err("Too many arguments".to_string()),
        }
    }

    fn execute(&self) -> Result<(), String> {
        match self.pkg_type {
            PackageType::Binary => bin::new_pkg(&self.path, &self.name),
            PackageType::Library => lib::new_pkg(&self.path, &self.name),
        }
    }
}

mod bin {
    use std::path::Path;

    use crate::fmt::success;

    const MAIN_C: &str = include_str!("../../template/main.c");
    const TAILOR_MANIFEST: &str = include_str!("../../template/bin/Tailor.toml");

    pub fn new_pkg(path: &Path, name: &str) -> Result<(), String> {
        if path.exists() {
            return Err(format!("destination `{}` already exists.", path.display()));
        }

        std::fs::create_dir_all(path.join("src")).map_err(|_| "fail to create src".to_string())?;

        std::fs::create_dir_all(path.join("include"))
            .map_err(|_| "fail to create include".to_string())?;

        std::fs::write(path.join("src/main.c"), MAIN_C)
            .map_err(|_| "fail to write src/main.c".to_string())?;

        std::fs::write(
            path.join("Tailor.toml"),
            TAILOR_MANIFEST.replace("$pkg_name", name),
        )
        .map_err(|_| "fail to write Tailor.toml".to_string())?;

        println!(
            "{} binary (application) package `{name}`",
            success("Creating")
        );

        Ok(())
    }
}

mod lib {
    use std::path::Path;

    use crate::fmt::success;

    const LIB_C: &str = include_str!("../../template/lib.c");
    const LIB_H: &str = include_str!("../../template/lib.h");
    const TAILOR_MANIFEST: &str = include_str!("../../template/lib/Tailor.toml");

    pub fn new_pkg(path: &Path, name: &str) -> Result<(), String> {
        if path.exists() {
            return Err(format!("destination `{}` already exists.", path.display()));
        }

        std::fs::create_dir_all(path.join("src")).map_err(|_| "fail to create src".to_string())?;

        std::fs::create_dir_all(path.join(format!("include/{name}/")))
            .map_err(|_| "fail to create include".to_string())?;

        std::fs::write(
            path.join(format!("src/{name}.c")),
            LIB_C.replace("$pkg_name", name),
        )
        .map_err(|_| format!("fail to write src/{name}.c"))?;

        std::fs::write(
            path.join(format!("include/{name}/{name}.h")),
            LIB_H
                .replace("$pkg_name_guard", &format!("{}_H", name.to_uppercase()))
                .replace("$pkg_name", name),
        )
        .map_err(|_| format!("fail to write include/{name}/{name}.h"))?;

        std::fs::write(
            path.join("Tailor.toml"),
            TAILOR_MANIFEST.replace("$pkg_name", name),
        )
        .map_err(|_| "fail to write Tailor.toml".to_string())?;

        println!("{} library package `{name}`", success("Creating"));

        Ok(())
    }
}
