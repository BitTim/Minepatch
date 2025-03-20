/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       mod.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   20.03.25, 11:30
 */

pub(crate) mod data;
mod error;
mod func;
mod msg;

pub use data::*;
pub use error::*;
pub use func::*;
pub use msg::*;
