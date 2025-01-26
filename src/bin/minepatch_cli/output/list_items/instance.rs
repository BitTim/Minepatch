/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       instance.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   25.01.25, 19:46
 */
use crate::output::format_bool;
use minepatch::instance::Instance;
use std::fs;
use tabled::Tabled;

#[derive(Debug, Tabled)]
pub(crate) struct InstanceListItem {
    #[tabled(rename = "Name")]
    pub(crate) name: String,
    #[tabled(rename = "Path")]
    pub(crate) path: String,
    #[tabled(rename = "Valid")]
    pub(crate) valid: String,
}

impl From<Instance> for InstanceListItem {
    fn from(value: Instance) -> Self {
        let valid = fs::exists(&value.path).unwrap_or_default();

        InstanceListItem {
            name: value.name.clone(),
            path: value.path.display().to_string(),
            valid: format_bool(&valid),
        }
    }
}
