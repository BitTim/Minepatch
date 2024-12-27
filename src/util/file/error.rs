/*
 * Copyright (c) 2024 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       error.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   27.12.24, 14:36
 */
use crate::util::error::ErrorType;

#[derive(Debug)]
pub enum FileError {
    PathNotFound,
    PathNoFileName,
    PathInvalidUTF8,
}

impl ErrorType for FileError {
    fn message(&self) -> &str {
        match self {
            FileError::PathNotFound => "Path not found",
            FileError::PathNoFileName => "Path does not include a file or directory name",
            FileError::PathInvalidUTF8 => "Path contains invalid UTF-8 formatting",
        }
    }

    fn hint(&self) -> &str {
        match self {
            FileError::PathNotFound => "Make sure that the path is properly formatted and the directory you are referencing actually exists.",
            FileError::PathNoFileName | FileError::PathInvalidUTF8 => "Try to specify the name manually with '-n' or '--name'."
        }
    }
}
