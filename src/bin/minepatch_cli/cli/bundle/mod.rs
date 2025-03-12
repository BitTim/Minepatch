/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       mod.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   10.03.25, 10:26
 */
mod create;
mod export;
mod import;
mod list;

pub(crate) use create::*;
pub(crate) use export::*;
pub(crate) use import::*;
pub(crate) use list::*;
use std::path::PathBuf;

use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum BundleCommands {
    /// List all mod packs.
    List {
        /// The name of the bundle.
        #[arg(short, long)]
        name: Option<String>,
    },

    /// Create a new bundle.
    Create {
        /// A unique name for the bundle.
        name: String,

        /// The description for this bundle.
        #[arg(short, long)]
        description: Option<String>,

        /// The template that is used for this bundle. An assigned template is purely informational and does not affect function.
        #[arg(short, long)]
        template: Option<String>,

        /// The path to a Minecraft instance. Using this option will generate a bundle from the contents of the instance.
        #[arg(short, long)]
        from: Option<PathBuf>,

        /// The name for the instance. Using this option will also link the instance provided by '-f' or '--from' to the specified name.
        #[arg(short, long, requires = "from")]
        instance: Option<String>,
    },

    /// Delete a bundle.
    Delete,

    /// Export a bundle to a file
    Export {
        /// Name of the bundle. Must be unique.
        name: String,

        /// Path of the exported file. Defaults to the current dir with <NAME>.mpb as filename if omitted.
        #[arg(short, long)]
        path: Option<PathBuf>,
    },

    /// Import a bundle from a file
    Import {
        /// Path to the file that should be imported
        path: PathBuf,

        /// Name for the imported bundle. If omitted, the exported name will be used. Must be unique.
        #[arg(short, long)]
        name: Option<String>,
    },
}
