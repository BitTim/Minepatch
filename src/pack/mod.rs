/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       mod.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   27.01.25, 10:53
 */
pub(super) mod data;
pub use data::Pack;
mod error;
pub use error::*;
mod func;
pub use func::*;
