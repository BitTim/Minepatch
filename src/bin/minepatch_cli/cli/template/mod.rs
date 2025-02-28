/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       mod.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   17.02.25, 19:42
 */
mod create;

use std::path::PathBuf;

mod export;
mod list;

pub(crate) use create::*;
pub(crate) use export::*;
pub(crate) use list::*;

use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum TemplateCommands {
    /// Creates a new template to be used with packs.
    Create {
        /// Name of the template. Must be unique.
        name: String,

        /// Loader that is used by the template. This does not impact the loaders used for the mods themselves.
        #[arg(short, long)]
        loader: Option<String>,

        /// Version of the template.
        #[arg(short, long)]
        version: Option<String>,

        /// A download link where the template can be downloaded from.
        #[arg(short, long)]
        download: Option<String>,
    },

    /// List all templates.
    List {
        /// Name of the template. Must be unique.
        #[arg(short, long)]
        name: Option<String>,
    },

    /// Export a template to a file
    Export {
        /// Name of the template. Must be unique.
        name: String,

        /// Path of the exported file. Defaults to the current dir with <NAME>.mpt as filename if omitted.
        #[arg(short, long)]
        path: Option<PathBuf>,
    },
}
