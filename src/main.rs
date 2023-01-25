extern crate core;

mod message;
mod cli;
mod progress_bar;
mod project_creation;
mod remote_repo;
mod disk;

use std::fmt::{Display, Formatter, write};
use std::thread::sleep;
use std::time::Duration;
use std::{io, vec};
use std::io::Write;
use std::path::{Path, PathBuf};
use clap::Parser;

use crate::message::Message;
use crate::progress_bar::ProgressBar;
use crate::cli::{Cli, CliCommand};
use crate::project_creation::ProjectBuilder;

#[derive(PartialEq, PartialOrd, Debug)]
pub enum TailorErr {
    NonEmptyDir,
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();

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
