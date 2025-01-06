/*
 * Copyright (c) 2024-2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       mod.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   06.01.25, 18:17
 */
use crate::common::output::format_bool;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use tabled::Tabled;

pub mod instance_cli;
pub mod instance_error;
pub mod instance_main;
mod instance_util;

#[derive(Debug, Tabled)]
pub(crate) struct InstanceDisplay {
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "Path")]
    path: String,
    #[tabled(rename = "Valid")]
    valid: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Instance {
    name: String,
    path: PathBuf,
}

impl Instance {
    pub(crate) fn new(name: &str, path: &Path) -> Self {
        Self {
            name: name.to_string(),
            path: path.to_path_buf(),
        }
    }

    pub(crate) fn get_name(&self) -> &str {
        &self.name
    }
    pub(crate) fn set_name(&mut self, name: &str) {
        self.name = name.to_string()
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
            valid: format_bool(&valid),
        }
    }
}
