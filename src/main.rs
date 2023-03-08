extern crate core;

mod cli;
mod disk;
mod error;
mod message;
mod progress_bar;
mod project_creation;
mod remote_repo;

use crate::cli::{Cli, CliCommand};
use crate::message::Message;
use crate::project_creation::ProjectBuilder;
use clap::Parser;

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

        if let Err(err) = builder.build().await {
            Message::fail(&format!(
                "Cannot create \"{name}\" project. Reason: {err:?}"
            ))
            .print();
        }
    }
}
