/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       base.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   13.01.25, 20:29
 */
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Base {
    name: String,
    version: String,
    link: String,
}
