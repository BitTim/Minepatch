/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       model.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   20.01.25, 15:19
 */
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct Pack {
    pub(crate) name: String,
    pub(crate) description: Option<String>,
    pub(crate) base: Option<String>,
    pub(crate) latest_patch: String,
}

impl Pack {
    pub(crate) fn empty(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            description: None,
            base: None,
            latest_patch: String::from("init"),
        }
    }

    pub(crate) fn new(
        name: &str,
        description: Option<String>,
        base: Option<String>,
        latest_patch: &str,
    ) -> Self {
        Self {
            name: name.to_owned(),
            description,
            base,
            latest_patch: latest_patch.to_owned(),
        }
    }
}
