/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       mod.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   08.02.25, 00:24
 */

pub(super) mod data;
mod error;
mod func;

pub use data::Mod;
pub use error::*;
pub use func::*;
