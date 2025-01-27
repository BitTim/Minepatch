/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       mod.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   27.01.25, 09:41
 */
pub(super) mod data;
pub use data::Template;
mod error;
pub use error::*;
mod func;

pub use func::*;
