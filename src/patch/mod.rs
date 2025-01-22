/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       mod.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   22.01.25, 16:40
 */
pub(crate) mod data;
pub use data::Patch;
mod error;
pub use error::*;
mod func;
pub use func::*;
