/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       mod.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   22.01.25, 02:17
 */
mod create;
pub(crate) use create::*;

use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum PackCommands {
    /// List all mod packs.
    List,

    /// Create a new mod pack.
    Create {
        /// A unique name for the mod pack.
        name: String,

        /// The description for this pack.
        #[arg(short, long)]
        description: Option<String>,

        /// The template that is used for this pack. An assigned template is purely informational and does not affect function.
        #[arg(short, long)]
        template: Option<String>,

        /// The path to a Minecraft instance. Using this option will generate a pack from the contents of the instance.
        #[arg(short, long)]
        from: Option<String>,

        /// The name for the instance. Using this option will also link the instance provided by '-f' or '--from' to the specified name.
        #[arg(short, long, requires = "from")]
        instance: Option<String>,
    },

    /// Delete a mod pack.
    Delete,
}
