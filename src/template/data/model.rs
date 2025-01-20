/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       model.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   20.01.25, 16:47
 */
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct Template {
    pub(crate) name: String,
    pub(crate) version: Option<String>,
    pub(crate) loader: Option<String>,
    pub(crate) download: Option<String>,
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
