/*
 * Copyright (c) 2024-2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       main.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   13.01.25, 20:29
 */
use clap::Parser;
use minepatch::commands::instance::instance_cli::InstanceCommands;
use minepatch::commands::instance::instance_main;
use minepatch::commands::pack::cli::PackCommands;
use minepatch::commands::vault::vault_cli::VaultCommands;
use minepatch::commands::vault::vault_main;
use minepatch::commands::{pack, update, Cli, Commands};
use minepatch::common::error;

fn match_command(command: &Commands) -> error::Result<()> {
    match command {
        Commands::Update => update::update()?,
        Commands::Instance {
            instance_commands: instance_command,
        } => match instance_command {
            InstanceCommands::List => instance_main::list()?,
            InstanceCommands::Link { path, name } => instance_main::link(path, name)?,
            InstanceCommands::Rename { name, new_name } => instance_main::rename(name, new_name)?,
            InstanceCommands::Unlink { name, all, yes } => instance_main::unlink(name, all, yes)?,
        },
        Commands::Vault {
            vault_commands: vault_command,
        } => match vault_command {
            VaultCommands::Add { path } => vault_main::add(path)?,
            VaultCommands::List { detailed, hash, id } => vault_main::list(detailed, hash, id)?,
            VaultCommands::Remove { hash, all, yes } => vault_main::remove(hash, all, yes)?,
        },
        Commands::Pack { pack_command } => match pack_command {
            PackCommands::List => {}
            PackCommands::Create {
                name,
                from,
                instance,
            } => pack::func::create::create(name, from, instance)?,
            PackCommands::Delete => {}
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
