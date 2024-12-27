/*
 * Copyright (c) 2024 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       instance_cli.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   27.12.24, 15:25
 */

use clap::Subcommand;
use std::path::PathBuf;

#[derive(Debug, Subcommand)]
pub enum InstanceCommands {
    /// List all linked instances
    List,

    /// Link an instance and assign a unique name
    Link {
        /// The path to the root folder of the instance
        path: PathBuf,

        /// Use this flag to assign a unique name. Defaults to the directory name.
        #[arg(short, long)]
        name: Option<String>,
    },

    /// Change the unique name of an already linked instance
    Rename {
        /// The current name of the instance
        name: String,

        /// Use this flag to assign a unique name. Defaults to the directory name.
        #[arg(short, long)]
        new_name: Option<String>,
    },

    /// Unlink an instance
    Unlink {
        /// The name of the instance
        name: String,
    },
}
