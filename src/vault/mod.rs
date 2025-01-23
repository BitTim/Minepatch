/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       mod.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   20.01.25, 13:46
 */

pub(crate) mod data;
pub use data::Mod;
mod error;
pub use error::*;
mod func;
pub use func::*;
