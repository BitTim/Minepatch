/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       instance.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   19.01.25, 13:09
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

impl TryFrom<Instance> for InstanceDisplay {
    type Error = ();

    fn try_from(value: Instance) -> Result<Self, Self::Error> {
        let valid = if let Ok(value) = fs::exists(&value.path) {
            value
        } else {
            false
        };

        Ok(InstanceDisplay {
            name: value.name.clone(),
            path: value.path.display().to_string(),
            valid: format_bool(&valid),
        })
    }
}
