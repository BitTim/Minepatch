/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       error.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   15.01.25, 11:24
 */
use crate::util::error::ErrorType;

#[derive(Debug)]
pub enum VaultError {
    NoLoaderDetected,
    HashNotFound,
}

impl ErrorType for VaultError {
    fn message(&self) -> String {
        match self {
            VaultError::NoLoaderDetected => {
                "No supported mod loader detected for the provided file"
            }
            VaultError::HashNotFound => "No mod with the specified hash has been found",
        }
        .to_owned()
    }

    fn hint(&self) -> String {
        match self {
            VaultError::NoLoaderDetected => "Make sure the file you provide is a valid mod for one of the supported mod loaders",
            VaultError::HashNotFound => "Make sure the hash you provided actually exists by using the 'vault list' subcommand",
        }
        .to_owned()
    }
}
