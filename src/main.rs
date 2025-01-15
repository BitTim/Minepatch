/*
 * Copyright (c) 2024-2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       main.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   15.01.25, 11:38
 */
use clap::Parser;
use minepatch::instance::cli::InstanceCommands;
use minepatch::pack::cli::PackCommands;
use minepatch::update::func;
use minepatch::util::cli::{Cli, Commands};
use minepatch::util::error;
use minepatch::vault::cli::VaultCommands;
use minepatch::vault::func::{add, list, remove};
use minepatch::{instance, pack};

fn match_command(command: &Commands) -> error::Result<()> {
    match command {
        Commands::Update => func::update::update()?,
        Commands::Instance {
            instance_commands: instance_command,
        } => match instance_command {
            InstanceCommands::List => {
                instance::func::list::list()?;
            }
            InstanceCommands::Link { path, name } => {
                instance::func::link::link(path, name)?;
            }
            InstanceCommands::Rename { name, new_name } => {
                instance::func::rename::rename(name, new_name)?;
            }
            InstanceCommands::Unlink { name, all, yes } => {
                instance::func::unlink::unlink(name, all, yes)?;
            }
        },
        Commands::Vault {
            vault_commands: vault_command,
        } => match vault_command {
            VaultCommands::Add { path, silent } => {
                add::add(path, silent)?;
            }
            VaultCommands::List { detailed, hash, id } => {
                list::list(detailed, hash, id)?;
            }
            VaultCommands::Remove {
                hash,
                all,
                yes,
                silent,
            } => {
                remove::remove(hash, all, yes, silent)?;
            }
        },
        Commands::Pack { pack_command } => match pack_command {
            PackCommands::List => {}
            PackCommands::Create {
                name,
                from,
                instance,
            } => {
                pack::func::create::create(name, from, instance)?;
            }
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
