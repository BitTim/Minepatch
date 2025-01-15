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
pub enum MetaError {
    MalformedMetaFile,
}

impl ErrorType for MetaError {
    fn message(&self) -> String {
        match self {
            MetaError::MalformedMetaFile => "Provided mod has a malformed metadata file",
        }
        .to_owned()
    }

    fn hint(&self) -> String {
        match self {
            MetaError::MalformedMetaFile => "Either the mod file is corrupt or you are using a mod for an unsupported mod loader",
        }.to_owned()
    }
}
