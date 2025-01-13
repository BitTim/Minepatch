/*
 * Copyright (c) 2024-2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       mod.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   13.01.25, 20:29
 */
use crate::commands::instance::instance_cli;
use crate::commands::pack::cli;
use crate::commands::vault::vault_cli;
use clap::{Parser, Subcommand};

pub mod instance;
pub mod pack;
pub mod update;
pub mod vault;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Check for updates. Downloads and applies update when found
    Update,

    /// Manage relevant Minecraft instances
    Instance {
        #[command(subcommand)]
        instance_commands: instance_cli::InstanceCommands,
    },

    /// Manage mod files present in vault
    Vault {
        #[command(subcommand)]
        vault_commands: vault_cli::VaultCommands,
    },

    /// Manage mod packs and their patches
    Pack {
        #[command(subcommand)]
        pack_command: cli::PackCommands,
    },
}
