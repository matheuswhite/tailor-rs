use crate::{
    absolute_path::AbsolutePath,
    command::{
        CommandIF,
        build_pkg::{BuildMode, BuildPkg},
    },
    external_tool::registry::Registry,
    fmt::success,
    manifest::{Manifest, package_type::PackageType},
    mode::Mode,
    package::Package,
};

use clap::Args;
use std::path::PathBuf;

#[derive(Debug, Args)]
#[command(about = "Build and run a Tailor binary package located at the specified path.", long_about = None)]
pub struct RunPkg {
    #[arg(long, value_enum, default_value_t = BuildMode::default())]
    #[arg(conflicts_with_all = &["debug", "release"], help = "Specify the build mode (debug or release).")]
    build_mode: BuildMode,

    #[arg(long, conflicts_with_all = &["release", "build_mode"], help = "Build the package in debug mode.")]
    debug: bool,
    #[arg(long, conflicts_with_all = &["debug", "build_mode"], help = "Build the package in release mode.")]
    release: bool,

    #[arg(
        help = "The path to the Tailor package to build and run. If not provided, the current directory is used.",
        default_value = "."
    )]
    path: String,
}

impl CommandIF for RunPkg {
    fn command(&self) -> Result<(), String> {
        let path: AbsolutePath = PathBuf::from(&self.path).try_into()?;
        let mode = if self.release || matches!(self.build_mode, BuildMode::Release) {
            Mode::Release
        } else {
            Mode::Debug
        };

        let mode_name = mode.to_string();
        let registry = Registry::default();

        let manifest_content = std::fs::read_to_string(path.inner().join("Tailor.toml"))
            .map_err(|_| "fail to read Tailor.toml")?;
        let manifest = Manifest::from_file(&manifest_content, &path)?;
        let pkg = Package::load_from_manifest(manifest, &registry)?;

        let pkg_type = pkg.manifest().pkg_type();
        let pkg_name = pkg.manifest().full_name();

        match pkg_type {
            PackageType::Library => {
                return Err("It's not possible run a library package".to_string());
            }
            PackageType::Binary => {
                BuildPkg::build(path.clone(), mode)?;

                let executable_path = path.inner().join("build").join(mode_name).join(&pkg_name);

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
            }
        };

        Ok(())
    }
}
