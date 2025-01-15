/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       pack.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   15.01.25, 13:44
 */
use crate::pack::data::base::Base;
use crate::pack::data::patch::Patch;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Pack {
    pub(crate) name: String,
    latest_version: String,
    base: Option<Base>,
    patches: Vec<Patch>,
}

impl Pack {
    pub(crate) fn empty(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            latest_version: String::from("init"),
            base: None,
            patches: vec![Patch::empty("init")],
        }
    }

    pub(crate) fn new(name: &str, base: Option<Base>, patches: &[Patch]) -> Self {
        Self {
            name: name.to_owned(),
            latest_version: String::from("init"),
            base,
            patches: Vec::from(patches),
        }
    }
}
