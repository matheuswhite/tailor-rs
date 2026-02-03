#![deny(warnings)]

mod absolute_path;
mod command;
mod config;
mod dependency_tree;
mod external_tool;
mod fmt;
mod manifest;
mod mode;
mod package;
mod storage;

use crate::command::{AppCommands, CommandIF};
use crate::config::Config;
use crate::fmt::error;

use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "tailor")]
#[command(about = "A tool for managing and maintaining monorepos.", long_about = None)]
struct CliCommands {
    #[command(subcommand)]
    command: AppCommands,
}

fn main() {
    if let Err(err) = Config::create_default_config() {
        eprintln!("\n{}: {}", error(), err);
        return;
    };

    let args = CliCommands::parse();

    dbg!(&args);
    if let Err(err) = args.command.command() {
        eprintln!("\n{}: {}", error(), err);
        std::process::exit(1);
    }
}
