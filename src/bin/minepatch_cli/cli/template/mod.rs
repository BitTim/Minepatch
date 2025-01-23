/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       mod.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   20.01.25, 16:56
 */
mod create;
pub(crate) use create::*;

use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum TemplateCommands {
    /// Creates a new template to be used with packs
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
}
