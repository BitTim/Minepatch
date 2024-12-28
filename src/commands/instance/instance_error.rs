/*
 * Copyright (c) 2024 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       instance_error.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   28.12.24, 13:53
 */
use crate::common::error::ErrorType;

#[derive(Debug)]
pub enum InstanceError {
    NameNotFound,
    NameTaken,
    NameNotChanged,
}

impl ErrorType for InstanceError {
    fn message(&self) -> String {
        match self {
            InstanceError::NameNotFound => "No instance found with this name",
            InstanceError::NameTaken => "Name is already used by another instance",
            InstanceError::NameNotChanged => "New name cannot be the same as old name",
        }
        .to_owned()
    }

    fn hint(&self) -> String {
        match self {
            InstanceError::NameNotFound => {
                "Available instances can be viewed with sub command 'instance list'"
            }
            InstanceError::NameTaken | InstanceError::NameNotChanged => {
                "Try specifying a different name with '-n' or '--name'"
            }
        }
        .to_owned()
    }
}
