/*
 * Copyright (c) 2024-2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       error.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   18.02.25, 15:52
 */
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FileError {
    #[error("Path not found: '{path}'")]
    PathNotFound { path: PathBuf },
    #[error("Path does not include a file or directory name : '{path}'")]
    PathNoFileName { path: PathBuf },
    #[error("Path contains invalid UTF-8 formatting: '{path}'")]
    PathInvalidUTF8 { path: PathBuf },
    #[error("Did not find the projects OS specific data folder")]
    DataPathError,
    #[error("Expected extension '{expected}' but got '{extension}' in '{path}'")]
    InvalidExtension {
        path: PathBuf,
        expected: String,
        extension: String,
    },
}
