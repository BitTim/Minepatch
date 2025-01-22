/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       mod.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   22.01.25, 03:32
 */
use clap::Subcommand;

mod create;
pub(crate) use create::*;
mod include;
pub(crate) use include::*;

#[derive(Subcommand, Debug)]
pub enum PatchCommands {
    /// Creates a new patch
    Create {
        /// Name of the patch.
        #[arg(short, long)]
        name: String,

        /// Name of the patch this one depends on.
        #[arg(short, long)]
        dependency: String,

        /// The hash of the folder after all changes from this patch
        #[arg(short, long)]
        state_hash: String,

        /// The pack this patch belongs to
        #[arg(short, long)]
        pack: String,
    },

    /// Includes a mod into a patch
    Include {
        /// Name of the patch.
        #[arg(short, long)]
        name: String,

        /// The pack this patch belongs to
        #[arg(short, long)]
        pack: String,

        /// The hash of the mod that should be added to this patch
        #[arg(short, long)]
        mod_hash: String,
    },
}
