/*
 * Copyright (c) 2024-2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       mod.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   20.01.25, 01:35
 */
use clap::{Parser, Subcommand};

pub(crate) mod instance;
pub(crate) mod pack;
pub(crate) mod vault;

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
