/*
 * Copyright (c) 2024-2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       mod.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   01.03.25, 00:53
 */
use clap::{Parser, Subcommand};

pub(crate) mod bundle;
pub(crate) mod instance;
pub(crate) mod patch;
pub(crate) mod template;
pub mod update;
pub(crate) mod vault;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Check for updates. Downloads and applies update when found.
    Update,

    /// Manage relevant Minecraft instances.
    Instance {
        #[command(subcommand)]
        instance_commands: instance::InstanceCommands,
    },

    /// Manage mod files present in vault.
    Vault {
        #[command(subcommand)]
        vault_commands: vault::VaultCommands,
    },

    /// Manage bundle templates.
    Template {
        #[command(subcommand)]
        template_commands: template::TemplateCommands,
    },

    /// Manage patches for mod packs.
    Patch {
        #[command(subcommand)]
        patch_commands: patch::PatchCommands,
    },

    /// Manage mod packs.
    Bundle {
        #[command(subcommand)]
        bundle_commands: bundle::BundleCommands,
    },
}
