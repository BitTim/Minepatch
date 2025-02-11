/*
 * Copyright (c) 2024-2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       lib.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   11.02.25, 03:58
 */
mod common;
pub use common::*;
mod error;
pub mod instance;
pub mod msg;
pub mod pack;
pub mod patch;
pub mod patch_with_mods;
pub mod prelude;
pub mod template;
pub mod update;
pub mod vault;
