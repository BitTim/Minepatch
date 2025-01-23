/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       error.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   22.01.25, 17:55
 */
use thiserror::Error;

#[derive(Error, Debug)]
pub enum VaultError {
    #[error("No supported mod loader detected for '{0}'")]
    NoLoaderDetected(String),
    #[error("No mod found with hash '{0}'")]
    NotFound(String),
}
