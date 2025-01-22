/*
 * Copyright (c) 2024-2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       mod.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   22.01.25, 17:02
 */
mod link;
pub(crate) use link::*;

use clap::Subcommand;
use std::path::PathBuf;

#[derive(Debug, Subcommand)]
pub enum InstanceCommands {
    // /// List all linked instances
    // List,
    /// Link an instance and assign a unique name
    Link {
        /// The path to the root folder of the instance
        path: PathBuf,

        /// Use this flag to assign a unique name. Defaults to the directory name.
        #[arg(short, long)]
        name: Option<String>,

        /// The pack this instance belongs to.
        #[arg(short = 'p', long)]
        pack: String,

        /// The currently applied patch of the specified pack.
        #[arg(short = 'a', long)]
        patch: String,
    },
    // /// Change the unique name of an already linked instance
    // Rename {
    //     /// The current name of the instance
    //     name: String,
    //
    //     /// Use this flag to assign a unique name. Defaults to the directory name.
    //     #[arg(short, long)]
    //     new_name: Option<String>,
    // },
    //
    // /// Unlink an instance
    // Unlink {
    //     /// The name of the instance
    //     #[arg(required_unless_present = "all")]
    //     name: Option<String>,
    //
    //     /// Use this flag instead of [NAME] to unlink all linked instances (This action cannot be reversed)
    //     #[arg(long, conflicts_with = "name")]
    //     all: bool,
    //
    //     /// Use this flag to skip the confirmation prompt
    //     #[arg(short, long)]
    //     yes: bool,
    // },
}
