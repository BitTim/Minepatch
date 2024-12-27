/*
 * Copyright (c) 2024 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       instance.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   26.12.24, 19:33
 */

use clap::Subcommand;
use std::path::PathBuf;

#[derive(Subcommand)]
pub enum Commands {
    /// List all linked instances
    List,

    /// Link an instance and assign a unique name
    Link {
        /// Use this flag to assign a unique name. Defaults to the directory name.
        #[arg(short, long)]
        name: Option<String>,

        /// The path to the root folder of the instance
        path: PathBuf,
    },

    /// Change the unique name of an already linked instance
    Relink {
        /// The current name of the instance
        #[arg(short, long)]
        old_name: String,

        /// The new name for the instance
        #[arg(short, long)]
        new_name: String,
    },

    /// Unlink an instance
    Unlink {
        /// The name of the instance
        #[arg(short, long)]
        name: String,
    },
}
