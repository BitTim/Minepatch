/*
 * Copyright (c) 2024-2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       lib.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   22.01.25, 19:08
 */
mod common;
pub use common::*;
mod error;
pub mod instance;
pub mod pack;
pub mod patch;
mod patch_with_mods;
pub mod prelude;
pub mod template;
pub mod update;
pub mod vault;
