/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       error.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   19.01.25, 13:27
 */
use thiserror::Error;

#[derive(Error, Debug)]
pub enum VaultError {
    #[error("No supported mod loader detected for '{0}'")]
    NoLoaderDetected(String),
    #[error("No mod found with hash '{0}'")]
    HashNotFound(String),
}
