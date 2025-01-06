/*
 * Copyright (c) 2024-2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       main.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   06.01.25, 00:56
 */
use clap::Parser;
use minepatch::commands::instance::instance_cli::InstanceCommands;
use minepatch::commands::instance::instance_main;
use minepatch::commands::vault::vault_cli::VaultCommands;
use minepatch::commands::vault::vault_main;
use minepatch::commands::{update, Cli, Commands};
use minepatch::common::error;

fn match_command(command: &Commands) -> error::Result<()> {
    match command {
        Commands::Update => update::update()?,
        Commands::Instance { instance_commands } => match instance_commands {
            InstanceCommands::List => instance_main::list()?,
            InstanceCommands::Link { path, name } => instance_main::link(path, name)?,
            InstanceCommands::Rename { name, new_name } => instance_main::rename(name, new_name)?,
            InstanceCommands::Unlink { name, all, yes } => instance_main::unlink(name, all, yes)?,
        },
        Commands::Vault { vault_commands } => match vault_commands {
            VaultCommands::Add { path } => vault_main::add(path)?,
            VaultCommands::Remove { hash } => vault_main::remove(hash)?,
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
