/*
 * Copyright (c) 2024-2025 Tim Anhalt (BitTim)
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
pub enum FileError {
    PathNotFound,
    PathNoFileName,
    PathInvalidUTF8,
}

impl ErrorType for FileError {
    fn message(&self) -> String {
        match self {
            FileError::PathNotFound => "Path not found",
            FileError::PathNoFileName => "Path does not include a file or directory name",
            FileError::PathInvalidUTF8 => "Path contains invalid UTF-8 formatting",
        }
        .to_owned()
    }

    fn hint(&self) -> String {
        match self {
            FileError::PathNotFound => "Make sure that the path is properly formatted and the directory you are referencing actually exists.",
            FileError::PathNoFileName | FileError::PathInvalidUTF8 => "Try to specify the name manually with '-n' or '--name'."
        }.to_owned()
    }
}
