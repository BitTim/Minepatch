/*
 * Copyright (c) 2024-2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       lib.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   01.03.25, 00:53
 */
mod common;
pub use common::*;
pub mod bundle;
mod error;
pub mod instance;
pub mod msg;
pub mod patch;
pub mod patch_with_mods;
pub mod prelude;
pub mod template;
pub mod vault;
