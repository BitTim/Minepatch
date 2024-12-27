/*
 * Copyright (c) 2024 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       instance.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   27.12.24, 01:53
 */
use crate::error::ErrorType;

#[derive(Debug)]
pub enum InstanceError {
    NameNotFound,
    NameTaken,
}

impl ErrorType for InstanceError {
    fn message(&self) -> &str {
        match self {
            InstanceError::NameNotFound => "No instance with this name found",
            InstanceError::NameTaken => "Name is already used by other instance",
        }
    }

    fn hint(&self) -> &str {
        match self {
            InstanceError::NameNotFound => "Check the name you specified for typos",
            InstanceError::NameTaken => "Try specifying a different name with '-n' or '--name'",
        }
    }
}
