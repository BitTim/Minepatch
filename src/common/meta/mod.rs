/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       mod.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   01.03.25, 18:55
 */
mod common;
pub(crate) mod data;
pub mod error;
mod fabric;
mod forge;
mod forge_legacy;

pub(crate) use common::*;
