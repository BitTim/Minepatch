/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       base.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   15.01.25, 18:55
 */
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct Base {
    name: String,
    version: String,
    link: String,
}
