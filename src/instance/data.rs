/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       data.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   15.01.25, 18:54
 */
use crate::instance::output::InstanceDisplay;
use crate::util::output::format_bool;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Eq, PartialEq, Debug, Serialize, Deserialize)]
pub(crate) struct Instance {
    pub(crate) name: String,
    pub(crate) path: PathBuf,
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
