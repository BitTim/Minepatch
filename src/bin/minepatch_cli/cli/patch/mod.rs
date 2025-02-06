/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       mod.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   06.02.25, 02:47
 */
use clap::Subcommand;

mod create;
mod exclude;
mod include;
mod list;
mod simulate;
mod view;

pub(crate) use create::*;
pub(crate) use exclude::*;
pub(crate) use include::*;
pub(crate) use list::*;
pub(crate) use simulate::*;
pub(crate) use view::*;

#[derive(Subcommand, Debug)]
pub enum PatchCommands {
    /// Creates a new patch.
    Create {
        /// Name of the patch.
        #[arg(short, long)]
        name: String,

        /// Name of the patch this one depends on.
        #[arg(short, long)]
        dependency: String,

        /// The hash of the folder after all changes from this patch.
        #[arg(short, long)]
        state_hash: String,

        /// The pack this patch belongs to.
        #[arg(short, long)]
        pack: String,
    },

    /// Includes a mod with a patch.
    Include {
        /// Name of the patch.
        #[arg(short, long)]
        name: String,

        /// The pack this patch belongs to.
        #[arg(short, long)]
        pack: String,

        /// The hash of the mod that should be added to this patch.
        #[arg(short, long)]
        mod_hash: String,
    },

    /// Excludes a mod with a patch.
    Exclude {
        /// Name of the patch.
        #[arg(short, long)]
        name: String,

        /// The pack this patch belongs to.
        #[arg(short, long)]
        pack: String,

        /// The hash of the mod that should be added to this patch.
        #[arg(short, long)]
        mod_hash: String,
    },

    /// Lists all patches matching the filters
    List {
        /// Name of the patch.
        #[arg(short, long)]
        name: Option<String>,

        /// The pack this patch belongs to.
        #[arg(short, long)]
        pack: Option<String>,
    },

    /// Simulates all patches up to and including the selected one.
    Simulate {
        /// Name of the patch.
        #[arg(short, long)]
        name: String,

        /// The pack this patch belongs to.
        #[arg(short, long)]
        pack: String,
    },

    /// Shows all details of a specific patch.
    View {
        /// Name of the patch.
        #[arg(short, long)]
        name: String,

        /// The pack this patch belongs to.
        #[arg(short, long)]
        pack: String,
    },
}
