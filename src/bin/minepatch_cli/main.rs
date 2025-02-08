/*
 * Copyright (c) 2024-2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       main.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   08.02.25, 11:14
 */
use crate::cli::instance::InstanceCommands;
use crate::cli::pack::PackCommands;
use crate::cli::patch::PatchCommands;
use crate::cli::template::TemplateCommands;
use crate::cli::{instance, pack, patch, template, vault, Cli, Commands};
use crate::output::status::{Status, StatusOutput};
use crate::output::Output;
use clap::Parser;
use cli::vault::VaultCommands;
use minepatch::db;
use minepatch::msg::Message;
use minepatch::prelude::*;
use minepatch::update::func;
use rusqlite::Connection;

mod cli;
mod output;

fn match_command(command: &Commands, connection: &Connection) -> Result<()> {
    match command {
        Commands::Update => func::update::update()?,
        Commands::Instance {
            instance_commands: instance_command,
        } => match instance_command {
            InstanceCommands::List { name } => instance::list(connection, name)?,
            InstanceCommands::Link {
                path,
                name,
                pack,
                patch,
            } => {
                instance::link(connection, path, name, pack, patch)?;
            }
        },
        Commands::Vault {
            vault_commands: vault_command,
        } => match vault_command {
            VaultCommands::Add { path, overwrite } => {
                vault::add(connection, path, overwrite)?;
            }
            VaultCommands::List {
                detailed,
                hash,
                id,
                name,
            } => {
                vault::list(connection, detailed, hash, id, name)?;
            }
            VaultCommands::Remove { hash, all, yes } => {
                vault::remove(connection, hash, *all, *yes)?;
            }
        },
        Commands::Template {
            template_commands: template_command,
        } => match template_command {
            TemplateCommands::Create {
                name,
                version,
                loader,
                download,
            } => template::create(connection, name, version, loader, download)?,
            TemplateCommands::List { name } => template::list(connection, name)?,
        },
        Commands::Patch {
            patch_commands: patch_command,
        } => match patch_command {
            PatchCommands::Create {
                name,
                dependency,
                pack,
            } => patch::create(connection, name, pack, dependency)?,
            PatchCommands::Exclude {
                name,
                pack,
                mod_hash,
            } => patch::exclude(connection, name, pack, mod_hash)?,
            PatchCommands::Generate { name, instance } => {
                patch::generate(connection, name, instance)?
            }
            PatchCommands::Include {
                name,
                pack,
                mod_hash,
            } => patch::include(connection, name, pack, mod_hash)?,
            PatchCommands::List { name, pack } => patch::list(connection, name, pack)?,
            PatchCommands::Simulate {
                name,
                pack,
                dir_hash,
            } => patch::simulate(connection, name, pack, dir_hash)?,
            PatchCommands::View { name, pack } => patch::view(connection, name, pack)?,
        },
        Commands::Pack {
            pack_commands: pack_command,
        } => match pack_command {
            PackCommands::List { name } => pack::list(connection, name)?,
            PackCommands::Create {
                name,
                description,
                template,
                from,
                instance,
            } => pack::create(connection, name, description, template, from, instance)?,
            PackCommands::Delete => {}
        },
    }

    Ok(())
}

fn error_handled() -> Result<()> {
    let mut connection = db::init()?;
    let tx = connection.transaction()?;

    let cli = Cli::parse();
    match_command(&cli.command, &tx)?;

    tx.commit()?;
    Ok(())
}

fn main() {
    if let Err(error) = error_handled() {
        StatusOutput::new(Status::Error, Message::new(&error.to_string())).print();
    }
}
