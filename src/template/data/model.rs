/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       model.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   27.01.25, 09:54
 */
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct Template {
    pub name: String,
    pub version: Option<String>,
    pub loader: Option<String>,
    pub download: Option<String>,
}

impl Template {
    pub(crate) fn new(
        name: &str,
        version: Option<String>,
        loader: Option<String>,
        download: Option<String>,
    ) -> Self {
        Self {
            name: name.to_owned(),
            version,
            loader,
            download,
        }
    }
}
