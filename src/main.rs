/*
 * Copyright (c) 2024 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       main.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   26.12.24, 00:09
 */

use minepatch::*;

use clap::Parser;
use cli::Cli;
use cli::{instance, Commands};

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Instance { instance_commands } => match instance_commands {
            instance::Commands::List => {
                if let Err(error) = core::instance::list() {
                    println!("{}", error);
                    std::process::exit(0);
                }
            }
            instance::Commands::Link { id, path } => {
                println!(
                    "PLACEHOLDER Linked instance \"{}\" at following path: \"{}\"",
                    id,
                    path.display()
                )
            }
            instance::Commands::Relink { old_id, new_id } => {
                println!(
                    "PLACEHOLDER Changed identifier for \"{}\" to \"{}\"",
                    old_id, new_id
                )
            }
            instance::Commands::Unlink { id } => {
                println!("PLACEHOLDER Unlinked instance \"{}\"", id)
            }
        },
    }
}
