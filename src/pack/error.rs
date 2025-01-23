/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       error.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   22.01.25, 02:37
 */
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PackError {
    #[error("No pack with name '{0}' was found.")]
    NotFound(String),
    #[error("Name '{0}' is already taken by another pack.")]
    NameTaken(String),
}
