/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       error.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   01.03.25, 00:53
 */
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BundleError {
    #[error("No bundle with name '{0}' was found.")]
    NotFound(String),
    #[error("Name '{0}' is already taken by another bundle.")]
    NameTaken(String),
}
