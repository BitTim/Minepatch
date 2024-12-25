/*
 * Copyright (c) 2024 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       instance.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   25.12.24, 03:04
 */

use clap::Subcommand;
use std::path::PathBuf;

#[derive(Subcommand)]
pub enum Commands {
    /// List all linked instances
    List,

    /// Link an instance and assign a unique identifier
    Link {
        /// A unique identifier that will be assigned to the instance
        #[arg(short, long)]
        id: String,

        /// The path to the root folder of the instance
        #[arg(short, long)]
        path: PathBuf,
    },

    /// Change the unique identifier of an already linked instance
    Relink {
        /// The current identifier of the instance
        #[arg(short, long)]
        old_id: String,

        /// The new identifier for the instance
        #[arg(short, long)]
        new_id: String,
    },

    /// Unlink an instance
    Unlink {
        /// The identifier of the instance
        #[arg(short, long)]
        id: String,
    },
}
