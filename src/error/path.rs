/*
 * Copyright (c) 2024 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       path.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   27.12.24, 01:45
 */
use crate::error::ErrorType;

#[derive(Debug)]
pub enum PathError {
    PathNotFound,
    PathNoFileName,
    PathInvalidUTF8,
}

impl ErrorType for PathError {
    fn message(&self) -> &str {
        match self {
            PathError::PathNotFound => "Path not found",
            PathError::PathNoFileName => "Path does not include a file or directory name",
            PathError::PathInvalidUTF8 => "Path contains invalid UTF-8 formatting",
        }
    }

    fn hint(&self) -> &str {
        match self {
            PathError::PathNotFound => "Make sure that the path is properly formatted and the directory you are referencing actually exists.",
            PathError::PathNoFileName | PathError::PathInvalidUTF8 => "Try to specify the name manually with '-n' or '--name'."
        }
    }
}
