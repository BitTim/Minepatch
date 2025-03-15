/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       mod.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   12.03.25, 11:43
 */
use clap::Subcommand;

mod create;
mod delete;
mod exclude;
mod generate;
mod include;
mod list;
mod rename;
mod simulate;
mod view;

pub(crate) use create::*;
pub(crate) use delete::*;
pub(crate) use exclude::*;
pub(crate) use generate::*;
pub(crate) use include::*;
pub(crate) use list::*;
pub(crate) use rename::*;
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

        /// The bundle this patch belongs to.
        #[arg(short, long)]
        bundle: String,
    },

    /// Excludes a mod with a patch.
    Exclude {
        /// Name of the patch.
        #[arg(short, long)]
        name: String,

        /// The bundle this patch belongs to.
        #[arg(short, long)]
        bundle: String,

        /// The hash of the mod that should be added to this patch.
        #[arg(short, long)]
        mod_hash: String,
    },

    /// Generate and apply a patch from changes on the file system for a specific instance.
    Generate {
        /// Name of the patch.
        #[arg(short, long)]
        name: String,

        /// The instance from which this patch will be generated.
        #[arg(short, long)]
        instance: String,
    },

    /// Includes a mod with a patch.
    Include {
        /// Name of the patch.
        #[arg(short, long)]
        name: String,

        /// The bundle this patch belongs to.
        #[arg(short, long)]
        bundle: String,

        /// The hash of the mod that should be added to this patch.
        #[arg(short, long)]
        mod_hash: String,
    },

    /// Lists all patches matching the filters
    List {
        /// Name of the patch.
        #[arg(short, long)]
        name: Option<String>,

        /// The bundle this patch belongs to.
        #[arg(short, long)]
        bundle: Option<String>,
    },

    /// Simulates all patches up to and including the selected one.
    Simulate {
        /// Name of the patch.
        #[arg(short, long)]
        name: String,

        /// The bundle this patch belongs to.
        #[arg(short, long)]
        bundle: String,

        /// Set this flag if you want to only know the resulting dir hash.
        #[arg(short, long)]
        dir_hash: bool,
    },

    /// Shows all details of a specific patch.
    View {
        /// Name of the patch.
        #[arg(short, long)]
        name: String,

        /// The bundle this patch belongs to.
        #[arg(short, long)]
        bundle: String,
    },

    /// Deletes a patch. Patch must not be a dependency or currently applied.
    Delete {
        /// Name of the patch.
        #[arg(short, long)]
        name: String,

        /// The bundle this patch belongs to.
        #[arg(short, long)]
        bundle: String,
    },

    /// Renames a patch.
    Rename {
        /// Name of the patch.
        #[arg(short, long)]
        name: String,

        /// The bundle this patch belongs to.
        #[arg(short, long)]
        bundle: String,

        /// The new name for the patch
        new_name: String,
    },
}
