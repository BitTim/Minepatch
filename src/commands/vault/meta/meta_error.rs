/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       meta_error.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   04.01.25, 18:31
 */
use crate::common::error::ErrorType;

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
