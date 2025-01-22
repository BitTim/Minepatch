/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       error.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   22.01.25, 17:05
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
}
