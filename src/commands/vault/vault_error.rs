/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       vault_error.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   05.01.25, 19:55
 */
use crate::common::error::ErrorType;

#[derive(Debug)]
pub enum VaultError {
    NoLoaderDetected,
}

impl ErrorType for VaultError {
    fn message(&self) -> String {
        match self {
            VaultError::NoLoaderDetected => {
                "No supported mod loader detected for the provided file"
            }
        }
        .to_owned()
    }

    fn hint(&self) -> String {
        match self {
            VaultError::NoLoaderDetected => {
                "Make sure the file you provide is a valid mod for one of the supported mod loaders"
            }
        }
        .to_owned()
    }
}
