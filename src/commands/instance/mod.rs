/*
 * Copyright (c) 2024 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       mod.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   27.12.24, 18:16
 */
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tabled::Tabled;

pub mod instance_cli;
pub mod instance_error;
mod instance_file;
pub mod instance_main;
mod instance_util;

#[derive(Debug, Tabled)]
#[tabled(rename_all = "PascalCase")]
pub(crate) struct InstanceDisplay {
    name: String,
    path: String,
    valid: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Instance {
    name: String,
    path: PathBuf,
}

impl Instance {
    pub(crate) fn new(name: String, path: PathBuf) -> Self {
        Self { name, path }
    }

    pub(crate) fn get_name(&self) -> String {
        self.name.to_string()
    }
    pub(crate) fn set_name(&mut self, name: String) {
        self.name = name
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
                true => "✓ Yes".green().to_string(),
                false => "✗  No".red().to_string(),
            },
        }
    }
}
