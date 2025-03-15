/*
 * Copyright (c) 2024-2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       error.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   12.03.25, 10:40
 */
use thiserror::Error;

#[derive(Error, Debug)]
pub enum InstanceError {
    #[error("No instance with name '{name}' was found.")]
    NameNotFound { name: String },
    #[error("Name '{name}' is already taken by another instance.")]
    NameTaken { name: String },
    #[error("New name cannot be the same as old name")]
    NameNotChanged,
    #[error(
        "States of the mod folder do not match.\n\tPresent dir hash: '{present_hash}'\n\tSimulated dir hash: '{sim_hash}'"
    )]
    StateMismatch {
        present_hash: String,
        sim_hash: String,
    },
    #[error("Failed to create symlink: '{src}' -> '{dest}'")]
    SymlinkFailed { src: String, dest: String },
    #[error("No patch was detected that matches the state of the instance '{dir_hash}'")]
    NoPatchDetected { dir_hash: String },
    #[error("Failed to update instance '{name}' to patch '{patch}'.")]
    PatchUpdateFailed { name: String, patch: String },
    #[error("No instance with applied patch '{patch}' was found.")]
    PatchNotFound { patch: String },
}
