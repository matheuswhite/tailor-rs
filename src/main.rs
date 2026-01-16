#![deny(warnings)]

mod absolute_path;
mod command;
mod config;
mod external_tool;
mod fmt;
mod manifest;
mod mode;
mod package;
mod storage;

use crate::command::clean_pkg::CleanPkg;
use crate::command::{build_pkg::BuildPkg, new_pkg::NewPkg, run_pkg::RunPkg};
use crate::config::Config;
use crate::{command::Command, fmt::error};
use std::env::args;

fn main() {
    if let Err(err) = Config::create_default_config() {
        eprintln!("\n{}: {}", error(), err);
        return;
    };

    let commands: &mut [&mut dyn Command] = &mut [
        &mut NewPkg::default(),
        &mut BuildPkg::default(),
        &mut RunPkg::default(),
        &mut CleanPkg::default(),
    ];
    let args = args().collect::<Vec<String>>();

    if args.contains(&"--help".to_owned()) || args.contains(&"-h".to_owned()) {
        help();
        return;
    }

    for cmd in commands {
        match cmd.parse_args(&args[1..]) {
            Ok(false) => continue,
            Ok(true) => {
                cmd.execute().unwrap_or_else(error_handling);
                return;
            }
            Err(err) => error_handling(err),
        }
    }

    help();
}

fn error_handling(err: String) {
    eprintln!("\n{}: {}", error(), err);
    std::process::exit(1);
}

fn help() {
    println!("C language package manager\n");
    println!("Usage: tailor [COMMAND] [OPTIONS] <path>\n");
    println!("Options:");
    println!("  --bin       Create a binary package (only for `new` command) (default)");
    println!("  --lib       Create a library package (only for `new` command)");
    println!("  --debug     Build (or run) in debug mode (default)");
    println!("  --release   Build (or run) in release mode\n");
    println!("Commands:");
    println!("  new         Create a new package");
    println!("  build       Build the package");
    println!("  run         Run the package");
    println!("  clean       Clean the build artifacts");
}
