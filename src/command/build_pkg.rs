use crate::{
    absolute_path::AbsolutePath,
    command::CommandIF,
    external_tool::{compiler::Compiler, registry::Registry},
    fmt::success,
    manifest::Manifest,
    mode::Mode,
    package::Package,
};

use clap::{Args, ValueEnum};
use std::{path::PathBuf, time::Instant};

#[derive(Default, Copy, Clone, Debug, ValueEnum)]
pub enum BuildMode {
    #[default]
    Debug,
    Release,
}

#[derive(Debug, Args)]
#[command(about = "Build a Tailor package located at the specified path.", long_about = None)]
pub struct BuildPkg {
    #[arg(long, value_enum, default_value_t = BuildMode::default())]
    #[arg(conflicts_with_all = &["debug", "release"], help = "Specify the build mode (debug or release).")]
    build_mode: BuildMode,

    #[arg(long, conflicts_with_all = &["release", "build_mode"], help = "Build the package in debug mode. [Default]")]
    debug: bool,
    #[arg(long, conflicts_with_all = &["debug", "build_mode"], help = "Build the package in release mode.")]
    release: bool,

    #[arg(
        help = "The path to the Tailor package to build. If not provided, the current directory is used.",
        default_value = "."
    )]
    path: String,
}

impl CommandIF for BuildPkg {
    fn command(&self) -> Result<(), String> {
        let path: AbsolutePath = PathBuf::from(&self.path).try_into()?;
        let mode = if self.release || matches!(self.build_mode, BuildMode::Release) {
            Mode::Release
        } else {
            Mode::Debug
        };

        BuildPkg::build(path, mode)
    }
}

impl BuildPkg {
    pub fn build(path: AbsolutePath, mode: Mode) -> Result<(), String> {
        let start = Instant::now();
        let registry = Registry::default();
        let manifest_content = std::fs::read_to_string(path.inner().join("Tailor.toml"))
            .map_err(|_| "fail to read Tailor.toml".to_string())?;
        let manifest = Manifest::from_file(&manifest_content, &path)?;
        let pkg = Package::load_from_manifest(manifest, &registry)?;

        let manifest = pkg.manifest();
        let pkg_type = manifest.pkg_type();
        let base_path = path.inner().join("build");
        let defines = pkg
            .options()
            .into_iter()
            .map(|def| def.to_define())
            .collect();

        let path = match mode {
            Mode::Debug => base_path.join("debug"),
            Mode::Release => base_path.join("release"),
        };
        std::fs::create_dir_all(&path)
            .map_err(|e| format!("fail to create build directory: {}", e))?;

        let compiler = Compiler::new(manifest.compiler(), manifest.full_name());

        compiler.build(mode, &path, pkg, pkg_type, defines)?;

        println!(
            "{} `{}` profile target in {:.2}s",
            success("Finished"),
            mode,
            start.elapsed().as_secs_f32()
        );

        Ok(())
    }
}
