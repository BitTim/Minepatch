/*
 * Copyright (c) 2024 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       main.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   27.12.24, 16:20
 */
use std::error::Error;

use clap::Parser;
use minepatch::commands::instance::instance_cli::InstanceCommands;
use minepatch::commands::instance::instance_logic;
use minepatch::commands::{Cli, Commands};

fn match_command(command: &Commands) -> Result<(), Box<dyn Error>> {
    match command {
        Commands::Instance { instance_commands } => match instance_commands {
            InstanceCommands::List => instance_logic::list()?,
            InstanceCommands::Link { path, name } => instance_logic::link(path, name)?,
            InstanceCommands::Rename { name, new_name } => instance_logic::rename(name, new_name)?,
            InstanceCommands::Unlink { name } => {
                println!("PLACEHOLDER Unlinked instance '{}'", name)
            }
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
