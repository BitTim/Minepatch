/*
 * Copyright (c) 2024-2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       mod.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   19.01.25, 14:04
 */
use clap::{Parser, Subcommand};

mod instance;
pub use instance::InstanceCommands;
mod pack;
pub use pack::PackCommands;
mod vault;
pub use vault::VaultCommands;

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
        instance_commands: instance::InstanceCommands,
    },

    /// Manage mod files present in vault
    Vault {
        #[command(subcommand)]
        vault_commands: vault::VaultCommands,
    },

    /// Manage mod packs and their patches
    Pack {
        #[command(subcommand)]
        pack_command: pack::PackCommands,
    },
}
