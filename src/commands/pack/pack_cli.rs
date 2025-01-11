/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       pack_cli.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   10.01.25, 20:04
 */

#[derive(Debug)]
pub enum PackCommands {
    /// List all mod packs
    List,

    /// Create a new mod pack
    Create {
        /// A unique name for the mod pack
        name: String,
    },

    /// Subset of commands to modify the mod pack

    /// Delete a mod pack
    Delete,
}
