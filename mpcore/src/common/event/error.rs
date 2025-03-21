/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       error.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   14.02.25, 18:57
 */
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EventError {
    #[error("No options were provided.")]
    NoOptions,
    #[error("Invalid selection, no data present at selected entry.")]
    InvalidSelection,
    #[error("Process has not been found, did you start it before ticking?")]
    ProcessNotFound,
}
