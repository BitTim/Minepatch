/*
 * Copyright (c) 2024-2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       error.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   19.01.25, 13:18
 */
use thiserror::Error;

#[derive(Error, Debug)]
pub enum InstanceError {
    #[error("No instance with name '{0}' was found.")]
    NameNotFound(String),
    #[error("Name '{0}' is already taken by another instance.")]
    NameTaken(String),
    #[error("New name cannot be the same as old name")]
    NameNotChanged,
}
