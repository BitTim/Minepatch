/*
 * Copyright (c) 2024-2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       error.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   19.01.25, 12:21
 */
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FileError {
    #[error("Path not found: '{0}'")]
    PathNotFound(String),
    #[error("Path does not include a file or directory name : '{0}'")]
    PathNoFileName(String),
    #[error("Path contains invalid UTF-8 formatting: '{0}'")]
    PathInvalidUTF8(String),
    #[error("Did not find the projects OS specific data folder")]
    DataPathError,
}
