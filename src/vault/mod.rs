/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       mod.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   11.02.25, 03:25
 */

pub(super) mod data;
mod error;
mod func;
mod msg;

pub use data::Mod;
pub use error::*;
pub use func::*;
pub use msg::*;
