/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       mod.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   11.02.25, 03:17
 */
pub(super) mod data;
pub use data::Pack;
mod error;
pub use error::*;
mod func;
mod msg;

pub use func::*;
pub use msg::*;
