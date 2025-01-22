/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       model.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   22.01.25, 15:07
 */
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct Pack {
    pub(crate) name: String,
    pub(crate) description: Option<String>,
    pub(crate) template: Option<String>,
}

impl Pack {
    pub fn new(name: &str, description: Option<String>, template: Option<String>) -> Self {
        Self {
            name: name.to_owned(),
            description,
            template,
        }
    }
}
