/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       output.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   15.01.25, 11:41
 */

use tabled::Tabled;

#[derive(Debug, Tabled)]
pub struct ModDisplay {
    #[tabled(rename = "Hash")]
    pub(crate) short_hash: String,
    #[tabled(rename = "Mod ID")]
    pub(crate) id: String,
    #[tabled(rename = "Name")]
    pub(crate) name: String,
    #[tabled(rename = "Version")]
    pub(crate) version: String,
    #[tabled(rename = "Loader")]
    pub(crate) loader: String,
    #[tabled(rename = "MC Version")]
    pub(crate) mc_version: String,
    #[tabled(rename = "Valid")]
    pub(crate) valid: String,
}
