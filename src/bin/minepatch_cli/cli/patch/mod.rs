/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       mod.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   20.01.25, 22:15
 */
use clap::Subcommand;

mod create;
pub(crate) use create::*;

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
}
