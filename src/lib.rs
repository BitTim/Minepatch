/*
 * Copyright (c) 2024-2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       lib.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   20.01.25, 22:33
 */
mod common;
pub use common::*;
mod error;
pub mod instance;
pub mod pack;
pub mod patch;
pub mod prelude;
pub mod template;
pub mod update;
pub mod vault;
