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

use crate::command::{build_pkg::BuildPkg, new_pkg::NewPkg, run_pkg::RunPkg};
use crate::config::Config;
use crate::{command::Command, fmt::error};
use std::env::args;

fn main() {
    Config::create_default_config();

    let commands: &mut [&mut dyn Command] = &mut [
        &mut NewPkg::default(),
        &mut BuildPkg::default(),
        &mut RunPkg::default(),
    ];
    let args = args().collect::<Vec<String>>();

    for cmd in commands {
        if cmd.parse_args(&args[1..]).is_some() {
            let res = cmd.execute();
            if let Err(e) = res {
                eprintln!("\n{}: {}", error(), e);
                std::process::exit(1);
            }
            return;
        }
    }

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
}
