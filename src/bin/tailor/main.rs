#![deny(warnings)]

pub mod commands;

use crate::commands::{AppCommands, CommandIF};
use tailor::config::Config;
use tailor::fmt::error;

use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "tailor")]
#[command(about = "A C package manager inspired by Rust's Cargo", long_about = None)]
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

    if let Err(err) = args.command.command() {
        eprintln!("\n{}: {}", error(), err);
        std::process::exit(1);
    }
}
