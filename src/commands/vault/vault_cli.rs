/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       vault_cli.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   06.01.25, 16:40
 */
use clap::Subcommand;
use std::path::PathBuf;

#[derive(Subcommand, Debug)]
pub enum VaultCommands {
    /// Adds a new mod file to the central mod vault
    Add {
        /// Path to the mod file
        path: PathBuf,
    },

    /// Lists all mods within the vault. Can be filtered and sorted.
    List {},

    /// Removes a mod from the vault
    Remove {
        /// Hash of the mod file that should be removed
        #[arg(required_unless_present = "all")]
        hash: Option<String>,

        /// Use this flag instead of [HASH] to remove all mods (This action cannot be reversed)
        #[arg(long, conflicts_with = "hash")]
        all: bool,

        /// Use this flag to skip the confirmation prompt
        #[arg(short, long)]
        yes: bool,
    },
}
