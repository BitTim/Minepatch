/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       error.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   08.02.25, 00:31
 */
use thiserror::Error;

#[derive(Error, Debug)]
pub enum VaultError {
    #[error("No mod found with hash '{hash}'.")]
    NotFound { hash: String },
    #[error("No hash has been provided.")]
    NoHashProvided,
    #[error("Hash '{hash}' is already used by a mod.")]
    HashTaken { hash: String },
    #[error("Linked file for hash '{hash}' at path '{path}' does not exist.")]
    PathNotExist { hash: String, path: String },

    #[error("No supported mod loader detected for file '{path}'.")]
    NoLoaderDetected { path: String },
    #[error("Mod at '{path}' with hash '{hash}' is already registered in vault.")]
    AlreadyExists { path: String, hash: String },
}
