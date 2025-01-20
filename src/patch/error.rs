/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       error.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   20.01.25, 22:38
 */
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PatchError {
    #[error("Name '{0}' is already used by another patch for pack '{1}'")]
    NameExists(String, String),
}
