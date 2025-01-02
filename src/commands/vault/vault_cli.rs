/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       vault_cli.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   02.01.25, 22:25
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
}
