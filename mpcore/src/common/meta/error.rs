/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       error.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   20.01.25, 18:07
 */
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MetaError {
    #[error("Provided mod has a malformed metadata file for the detected loader: '{0}'")]
    MalformedMetaFile(String),
}
