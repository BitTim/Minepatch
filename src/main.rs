/*
 * Copyright (c) 2024 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       main.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   27.12.24, 02:41
 */
use minepatch::*;
use std::error::Error;

use clap::Parser;
use cli::Cli;
use cli::{instance, Commands};

fn match_command(command: &Commands) -> Result<(), Box<dyn Error>> {
    match command {
        Commands::Instance { instance_commands } => match instance_commands {
            instance::Commands::List => logic::instance::list()?,
            instance::Commands::Link { name, path } => logic::instance::link(name, path)?,
            instance::Commands::Relink { old_name, new_name } => {
                println!(
                    "PLACEHOLDER Changed identifier for \"{}\" to \"{}\"",
                    old_name, new_name
                )
            }
            instance::Commands::Unlink { name } => {
                println!("PLACEHOLDER Unlinked instance \"{}\"", name)
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
