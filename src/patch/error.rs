/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       error.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   08.02.25, 21:57
 */
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PatchError {
    #[error("No patch with name '{name}' was found for pack '{pack}'.")]
    NotFound { name: String, pack: String },
    #[error("No patch with dependency '{dependency}' was found for pack '{pack}'.")]
    DepNotFound { dependency: String, pack: String },
    #[error("No patches for pack '{pack}' were found.")]
    PackNotFound { pack: String },
    #[error("No patches with dir state '{src_dir_hash}' found for pack '{pack}'")]
    SrcDirHashNotFound { src_dir_hash: String, pack: String },
    #[error("Name '{name}' is already used by another patch for pack '{pack}'")]
    NameExists { name: String, pack: String },

    #[error(
        "Mod with hash '{hash}' is already included in pack '{pack}' at the time of patch '{name}'"
    )]
    ModIncluded {
        hash: String,
        pack: String,
        name: String,
    },
    #[error(
        "Mod with hash '{hash}' is not present in pack '{pack}' at the time of patch '{name}'"
    )]
    ModExcluded {
        hash: String,
        pack: String,
        name: String,
    },

    #[error("Mod with hash '{hash}' is already included in patch '{name}' for pack '{pack}'")]
    RelTaken {
        hash: String,
        name: String,
        pack: String,
    },
    #[error("Mod with hash '{hash}' is not associated with patch '{name}' for pack '{pack}'")]
    RelNotFound {
        hash: String,
        name: String,
        pack: String,
    },
    #[error("Patch '{name}' for pack '{pack}' does not have any associated mods")]
    RelEmpty { name: String, pack: String },
}
