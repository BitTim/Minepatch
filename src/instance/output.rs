/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       output.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   13.01.25, 21:41
 */

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
