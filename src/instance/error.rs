/*
 * Copyright (c) 2024-2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       error.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   23.01.25, 17:57
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
    #[error("Cannot link instance to specified patch, required state does not match.\n\tPresent state: '{0}'\n\tSimulated state: '{1}'"
    )]
    StateMismatch(String, String),
    #[error("Failed to create symlink: '{0}' -> '{1}'")]
    SymlinkFailed(String, String),
}
