/*
 * Copyright (c) 2024 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       main.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   28.12.24, 02:06
 */
use clap::Parser;
use minepatch::commands::instance::instance_cli::InstanceCommands;
use minepatch::commands::instance::instance_main;
use minepatch::commands::{Cli, Commands};
use minepatch::common::error;

fn match_command(command: &Commands) -> error::Result<()> {
    match command {
        Commands::Instance { instance_commands } => match instance_commands {
            InstanceCommands::List => instance_main::list()?,
            InstanceCommands::Link { path, name } => instance_main::link(path, name)?,
            InstanceCommands::Rename { name, new_name } => instance_main::rename(name, new_name)?,
            InstanceCommands::Unlink { name, all, yes } => instance_main::unlink(name, all, yes)?,
        },
    }

    Ok(())
}

fn main() {
    let cli = Cli::parse();

    if let Err(error) = match_command(&cli.command) {
        println!("{}", error);
    }
}
