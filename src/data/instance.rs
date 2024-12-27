/*
 * Copyright (c) 2024 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       instance.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   26.12.24, 20:52
 */
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tabled::Tabled;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Instance {
    name: String,
    path: PathBuf,
}

#[derive(Debug, Tabled)]
#[tabled(rename_all = "PascalCase")]
pub(crate) struct InstanceDisplay {
    name: String,
    path: String,
    valid: String,
}

impl Instance {
    pub(crate) fn new(name: String, path: PathBuf) -> Self {
        Self { name, path }
    }

    pub(crate) fn get_name(&self) -> String {
        self.name.to_string()
    }

    pub(crate) fn to_display(&self) -> InstanceDisplay {
        let valid = if let Ok(value) = fs::exists(&self.path) {
            value
        } else {
            false
        };

        InstanceDisplay {
            name: self.name.clone(),
            path: self.path.display().to_string(),
            valid: match valid {
                true => "✔".green().to_string(),
                false => "✘".red().to_string(),
            },
        }
    }
}
