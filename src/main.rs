extern crate core;

mod message;
mod cli;
mod progress_bar;
mod project_creation;
mod remote_repo;
mod disk;

use clap::Parser;
use crate::cli::{Cli, CliCommand};
use crate::project_creation::ProjectBuilder;

#[derive(PartialEq, PartialOrd, Debug)]
pub enum TailorErr {
    NonEmptyDir,
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    #[allow(irrefutable_let_patterns)]
    if let CliCommand::New { git, force, name } = args.command {
        let mut builder = ProjectBuilder::new(&name);

        if force {
            builder.enable_overwrite();
        };

        if git {
            builder.enable_git();
        }

        builder.build().await;
    }
}
