/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       mod.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   02.01.25, 22:25
 */
use serde::{Deserialize, Serialize};

pub mod vault_cli;
pub mod vault_main;

#[derive(Debug, Serialize, Deserialize)]
pub struct Mod {}
