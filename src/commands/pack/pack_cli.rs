/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       pack_cli.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   09.01.25, 21:49
 */

#[derive(Debug)]
pub enum PackCommands {
    /// List all mod packs
    List,

    /// Create a new mod pack
    Create,

    /// Delete a mod pack
    Delete,

    /// Add a mod to be included in a mod pack
    Include,

    /// Remove a mod from a mod pack
    Exclude,

    /// Apply all changes to a mod pack and save it as a patch
    Patch,
}
