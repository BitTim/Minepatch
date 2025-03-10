/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       error.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   10.03.25, 10:26
 */
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PatchError {
    #[error("No patch with name '{name}' was found for bundle '{bundle}'.")]
    NotFound { name: String, bundle: String },
    #[error("No patch with dependency '{dependency}' was found for bundle '{bundle}'.")]
    DepNotFound { dependency: String, bundle: String },
    #[error("No patches for bundle '{bundle}' were found.")]
    BundleNotFound { bundle: String },
    #[error("No patches with dir state '{src_dir_hash}' found for bundle '{bundle}'")]
    SrcDirHashNotFound {
        src_dir_hash: String,
        bundle: String,
    },
    #[error("Name '{name}' is already used by another patch for bundle '{bundle}'")]
    NameExists { name: String, bundle: String },

    #[error(
        "Mod with hash '{hash}' is already included in bundle '{bundle}' at the time of patch '{name}'"
    )]
    ModIncluded {
        hash: String,
        bundle: String,
        name: String,
    },
    #[error(
        "Mod with hash '{hash}' is not present in bundle '{bundle}' at the time of patch '{name}'"
    )]
    ModExcluded {
        hash: String,
        bundle: String,
        name: String,
    },

    #[error("Mod with hash '{hash}' is already included in patch '{name}' for bundle '{bundle}'")]
    RelTaken {
        hash: String,
        name: String,
        bundle: String,
    },
    #[error("Mod with hash '{hash}' is not associated with patch '{name}' for bundle '{bundle}'")]
    RelNotFound {
        hash: String,
        name: String,
        bundle: String,
    },
    #[error("Patch '{name}' for bundle '{bundle}' does not have any associated mods")]
    RelEmpty { name: String, bundle: String },
}
