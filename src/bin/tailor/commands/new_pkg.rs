use crate::commands::CommandIF;

use clap::{Args, ValueEnum};
use std::path::PathBuf;

#[derive(Default, Copy, Clone, Debug, ValueEnum)]
enum PkgType {
    #[default]
    Bin,
    Lib,
}

#[derive(Debug, Args)]
#[command(about = "Create a new Tailor package at the specified path.", long_about = None)]
pub struct NewPkg {
    #[arg(long, value_enum, default_value_t = PkgType::default())]
    #[arg(conflicts_with_all = &["lib", "bin"], help = "Specify the package type (binary or library).")]
    pkg_type: PkgType,

    #[arg(long, conflicts_with_all = &["lib", "pkg_type"], help = "Create a binary (application) package. [Default]")]
    bin: bool,
    #[arg(long, conflicts_with_all = &["bin", "pkg_type"], help = "Create a library package.")]
    lib: bool,

    #[arg(required = true)]
    path: String,
}

impl CommandIF for NewPkg {
    fn command(&self) -> Result<(), String> {
        let path = PathBuf::from(&self.path);
        let name = path
            .file_name()
            .and_then(|s| s.to_str())
            .map(String::from)
            .unwrap_or_default();

        if self.lib || matches!(self.pkg_type, PkgType::Lib) {
            lib::new_pkg(&path, &name)
        } else {
            bin::new_pkg(&path, &name)
        }
    }
}

mod bin {
    use std::path::Path;

    use tailor::fmt::success;

    const MAIN_C: &str = include_str!("../../../../template/main.c");
    const TAILOR_MANIFEST: &str = include_str!("../../../../template/bin/Tailor.toml");

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

    use tailor::fmt::success;

    const LIB_C: &str = include_str!("../../../../template/lib.c");
    const LIB_H: &str = include_str!("../../../../template/lib.h");
    const TAILOR_MANIFEST: &str = include_str!("../../../../template/lib/Tailor.toml");

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
