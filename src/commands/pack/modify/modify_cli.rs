/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       modify_cli.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   11.01.25, 13:44
 */

#[derive(Debug)]
pub enum PackModifyCommands {
    /// Add a mod to the mod pack
    Add {
        /// The full hash of the mod to add
        hash: String,
    },

    /// Remove a mod from the mod pack
    Remove {
        /// The full hash of the mod to remove
        hash: String,
    },

    /// Save all changes since the last patch as a new patch
    Patch {
        /// Unique version name of the patch
        name: String,
    },

    /// Change the mod packs metadata
    Meta {
        /// Minecraft version the mod pack is made for
        #[arg(short, long)]
        mc_version: Option<String>,

        /// Mod loader the mod pack is using
        #[arg(short, long)]
        loader: Option<String>,
    },
}
