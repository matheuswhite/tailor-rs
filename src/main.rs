#![deny(warnings)]

mod cmake;
mod commands;
mod dependency;
mod dependency_manager;
mod fmt;
mod git;
mod mode;
mod package;

use std::env::args;

use crate::{
    commands::build_pkg::BuildPkg, commands::clean_pkg::CleanPkg, commands::command::Command,
    commands::new_pkg::NewPkg, commands::run_pkg::RunPkg, fmt::error,
};

fn main() {
    let commands: &mut [&mut dyn Command] = &mut [
        &mut NewPkg::default(),
        &mut BuildPkg::default(),
        &mut RunPkg::default(),
        &mut CleanPkg::default(),
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
