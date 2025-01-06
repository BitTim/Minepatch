/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       vault_cli.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   06.01.25, 00:55
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

    /// Removes a mod from the vault
    Remove {
        /// Hash of the mod file that should be removed
        hash: String,
    },
}
