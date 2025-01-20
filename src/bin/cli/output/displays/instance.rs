/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       instance.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   20.01.25, 02:13
 */
use crate::output::format_bool;
use minepatch::instance::data::Instance;
use std::fs;
use tabled::Tabled;

#[derive(Debug, Tabled)]
pub(crate) struct InstanceDisplay {
    #[tabled(rename = "Name")]
    pub(crate) name: String,
    #[tabled(rename = "Path")]
    pub(crate) path: String,
    #[tabled(rename = "Valid")]
    pub(crate) valid: String,
}

impl From<Instance> for InstanceDisplay {
    fn from(value: Instance) -> Self {
        let valid = if let Ok(value) = fs::exists(&value.path) {
            value
        } else {
            false
        };

        InstanceDisplay {
            name: value.name.clone(),
            path: value.path.display().to_string(),
            valid: format_bool(&valid),
        }
    }
}
