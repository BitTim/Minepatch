/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       mod.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   02.03.25, 00:17
 */
use clap::Subcommand;
use std::path::PathBuf;

mod add;
mod export;
mod import;
mod list;
mod remove;

pub(crate) use add::*;
pub(crate) use export::*;
pub(crate) use import::*;
pub(crate) use list::*;
pub(crate) use remove::*;

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

        /// Only display entries where the name contains the provided name.
        #[arg(short, long)]
        name: Option<String>,
    },

    /// Adds a new mod file to the central mod vault.
    Add {
        /// Path to the mod file.
        path: PathBuf,

        /// When this flag is set and a mod file with the same hash is already registered, said file will be overwritten
        #[arg(short, long)]
        overwrite: bool,
    },

    /// Removes a mod from the vault
    Remove {
        /// Hash of the mod file that should be removed.
        #[arg(required_unless_present = "all")]
        hash: Option<String>,

        /// Use this flag instead of [HASH] to remove all mods (This action cannot be reversed).
        #[arg(long, conflicts_with = "hash")]
        all: bool,

        /// Use this flag to skip the confirmation prompt.
        #[arg(short, long)]
        yes: bool,
    },

    /// Export a mod from the vault
    Export {
        /// Hash of the mod file to be exported
        hash: String,

        /// Path of the resulting .mpm file
        #[arg(short, long)]
        path: Option<PathBuf>,
    },

    /// Import a mod into the vault
    Import {
        /// Path of the file to be imported
        path: PathBuf,
    },
}
