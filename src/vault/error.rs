/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       error.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   04.02.25, 23:19
 */
use thiserror::Error;

#[derive(Error, Debug)]
pub enum VaultError {
    #[error("No supported mod loader detected for '{0}'")]
    NoLoaderDetected(String),
    #[error("No mod found with hash '{0}'")]
    NotFound(String),
    #[error("Hash '{0}' is already used by a mod.")]
    HashTaken(String),
}
