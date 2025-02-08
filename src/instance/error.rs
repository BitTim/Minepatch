/*
 * Copyright (c) 2024-2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       error.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   08.02.25, 02:37
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
    #[error(
        "States of the mod folder do not match.\n\tPresent state: '{0}'\n\tSimulated state: '{1}'"
    )]
    StateMismatch(String, String),
    #[error("Failed to create symlink: '{0}' -> '{1}'")]
    SymlinkFailed(String, String),
}
