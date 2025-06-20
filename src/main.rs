#![deny(warnings)]

mod build_pkg;
mod cmake;
mod command;
mod dependency;
mod dependency_manager;
mod fmt;
mod git;
mod mode;
mod new_pkg;
mod package;
mod run_pkg;

use std::env::args;

use crate::{build_pkg::BuildPkg, command::Command, fmt::error, new_pkg::NewPkg, run_pkg::RunPkg};

fn main() {
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
