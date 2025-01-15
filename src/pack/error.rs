/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       error.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   15.01.25, 13:50
 */
use crate::util::error::ErrorType;
use std::fmt::Debug;

#[derive(Debug)]
pub enum PackError {
    PackNotFound,
    PackExists,
}

impl ErrorType for PackError {
    fn message(&self) -> String {
        match self {
            PackError::PackNotFound => "A pack with this name was not found",
            PackError::PackExists => "Pack with this name already exists",
        }
        .to_owned()
    }

    fn hint(&self) -> String {
        match self {
            PackError::PackNotFound => "View existing pack names with the 'pack list' command.",
            PackError::PackExists => "Try specifying another name that is not used yet.",
        }
        .to_owned()
    }
}
