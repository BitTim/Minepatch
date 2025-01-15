/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       cli.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   15.01.25, 11:57
 */
use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum PackCommands {
    /// List all mod packs.
    List,

    /// Create a new mod pack.
    Create {
        /// A unique name for the mod pack.
        name: String,

        /// The path to a Minecraft instance. Using this option will generate a pack from the contents of the instance.
        #[arg(short, long)]
        from: Option<String>,

        /// The name for the instance. Using this option will also link the instance provided by '-f' or '--from' to the specified name.
        #[arg(short, long, requires = "from")]
        instance: Option<String>,

        /// This option suppresses output to the console.
        #[arg(short, long)]
        silent: bool,
    },

    /// Delete a mod pack.
    Delete,
}