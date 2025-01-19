/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       cli.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   15.01.25, 14:55
 */
use clap::Subcommand;
use std::path::PathBuf;

#[derive(Subcommand, Debug)]
pub enum VaultCommands {
    /// Lists all mods within the vault. Can be filtered and sorted.
    List {
        /// Displays all matching entries in a more detailed view with all data visible.
        #[arg(short, long)]
        detailed: bool,

        /// Only display entries where the hash contains the provided hash.
        #[arg(short = 'a', long)]
        hash: Option<String>,

        /// Only display entries where the id contains the provided mod id.
        #[arg(short, long)]
        id: Option<String>,
    },

    /// Adds a new mod file to the central mod vault.
    Add {
        /// Path to the mod file.
        path: PathBuf,

        /// This flag suppresses output to the console.
        #[arg(short, long)]
        silent: bool,

        /// When this flag is set and a mod file with the same hash is already registered, said file will be overwritten
        #[arg(short, long)]
        overwrite: bool,
    },

    /// Removes a mod from the vault
    Remove {
        /// Full hash of the mod file that should be removed.
        #[arg(required_unless_present = "all")]
        hash: Option<String>,

        /// Use this flag instead of [HASH] to remove all mods (This action cannot be reversed).
        #[arg(long, conflicts_with = "hash")]
        all: bool,

        /// Use this flag to skip the confirmation prompt.
        #[arg(short, long)]
        yes: bool,

        /// This flag suppresses output to the console. Only works if '-y' or '--yes' is set.
        #[arg(short, long, requires = "yes")]
        silent: bool,
    },
}
