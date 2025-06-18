#![deny(warnings)]

mod build_pkg;
mod cmake;
mod command;
mod dependency;
mod dependency_manager;
mod git;
mod mode;
mod new_pkg;
mod package;
mod run_pkg;

use std::env::args;

use crate::{build_pkg::BuildPkg, command::Command, new_pkg::NewPkg, run_pkg::RunPkg};

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
                eprintln!("Error: {}", e);
            }
            return;
        }
    }

    eprintln!("Usage: new <path> [--bin | --lib]");
}
