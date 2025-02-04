/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       error.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   04.02.25, 21:43
 */
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PatchError {
    #[error("No patch with name '{0}' was found for pack '{1}'.")]
    NotFound(String, String),
    #[error("Name '{0}' is already used by another patch for pack '{1}'")]
    NameExists(String, String),

    #[error("Mod with hash '{0}' is already included in patch '{1}' for pack '{2}'")]
    ModAlreadyIncluded(String, String, String),
    #[error("Mod with hash '{0}' is not associated with patch '{1}' for pack '{2}'")]
    RelNotFound(String, String, String),
    #[error("Patch '{0}' for pack '{1}' does not have any associated mods")]
    RelEmpty(String, String),
}
