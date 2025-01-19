/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       error.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   19.01.25, 13:20
 */
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PackError {
    #[error("No pack with name '{0}' was found.")]
    PackNotFound(String),
    #[error("Name '{0}' is already taken by another pack.")]
    PackExists(String),
}
