/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       mod.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   03.01.25, 23:20
 */
use serde::{Deserialize, Serialize};

pub mod meta;
pub mod vault_cli;
mod vault_error;
pub mod vault_main;

#[derive(Debug, Serialize, Deserialize)]
pub struct Mod {}
