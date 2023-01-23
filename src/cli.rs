use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "tailor")]
#[command(bin_name = "tailor")]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: CliCommand,
}

#[derive(Subcommand)]
pub enum CliCommand {
    /// Create a new hat project
    New {
        /// Initialize git and add a basic gitignore for the project
        #[arg(long, default_value_t = true)]
        git: bool,
        /// If the directory is not empty, then create the project anyway
        #[arg(short, default_value_t = false)]
        force: bool,
        /// Project name
        name: String,
    },
}
