use clap::Subcommand;

pub mod build_pkg;
pub mod clean_pkg;
pub mod new_pkg;
pub mod run_pkg;

#[derive(Debug, Subcommand)]
pub enum AppCommands {
    New(new_pkg::NewPkg),
    Build(build_pkg::BuildPkg),
    Run(run_pkg::RunPkg),
    Clean(clean_pkg::CleanPkg),
}

impl CommandIF for AppCommands {
    fn command(&self) -> Result<(), String> {
        match self {
            AppCommands::New(cmd) => cmd.command(),
            AppCommands::Build(cmd) => cmd.command(),
            AppCommands::Run(cmd) => cmd.command(),
            AppCommands::Clean(cmd) => cmd.command(),
        }
    }
}

pub trait CommandIF {
    fn command(&self) -> Result<(), String>;
}
