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
mod apply;
mod link;
mod list;

pub(crate) use apply::*;
pub(crate) use link::*;
pub(crate) use list::*;

use clap::Subcommand;
use std::path::PathBuf;

#[derive(Debug, Subcommand)]
pub enum InstanceCommands {
    /// Apply a specific patch to the specified instance.
    Apply {
        /// The instance the patch is applied to.
        #[arg(short, long)]
        name: String,

        /// The patch that should be applied.
        #[arg(short, long)]
        patch: String,
    },

    /// List all linked instances.
    List {
        /// Specifies a name to filter for.
        #[arg(short, long)]
        name: Option<String>,
    },

    /// Link an instance and assign a unique name
    Link {
        /// The path to the root folder of the instance
        #[arg(short = 'f', long)]
        path: PathBuf,

        /// Use this flag to assign a unique name. Defaults to the directory name.
        #[arg(short, long)]
        name: Option<String>,

        /// The bundle this instance belongs to. Tries to detect if omitted.
        #[arg(short = 'p', long)]
        bundle: Option<String>,
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
