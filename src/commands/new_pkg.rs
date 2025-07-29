use std::{path::PathBuf, str::FromStr};

use crate::commands::command::Command;
use crate::package::PackageType;

#[derive(Default)]
pub struct NewPkg {
    path: PathBuf,
    name: String,
    pkg_type: PackageType,
}

impl Command for NewPkg {
    fn parse_args(&mut self, args: &[String]) -> Option<()>
    where
        Self: Sized,
    {
        match args.len() {
            2 => {
                if args[0] != "new" {
                    return None;
                }

                self.path = PathBuf::from_str(&args[1]).ok()?;
                self.name = self
                    .path
                    .file_name()
                    .and_then(|s| s.to_str())
                    .map(String::from)
                    .unwrap_or_default();

                Some(())
            }
            3 => {
                if args[0] != "new" {
                    return None;
                }

                match args[1].as_str() {
                    "--bin" => self.pkg_type = PackageType::Binary,
                    "--lib" => self.pkg_type = PackageType::Library,
                    _ => return None,
                }

                self.path = PathBuf::from_str(&args[2]).ok()?;
                self.name = self
                    .path
                    .file_name()
                    .and_then(|s| s.to_str())
                    .map(String::from)
                    .unwrap_or_default();

                Some(())
            }
            _ => None,
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
        let abs_path = Path::new(".")
            .canonicalize()
            .map_err(|_| "fail to get absolute path")?
            .join(path);
        if abs_path.exists() {
            return Err(format!(
                "destination `{}` already exists.",
                abs_path.display()
            ));
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
        let abs_path = Path::new(".")
            .canonicalize()
            .map_err(|_| "fail to get absolute path")?
            .join(path);
        if abs_path.exists() {
            return Err(format!(
                "destination `{}` already exists.",
                abs_path.display()
            ));
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
