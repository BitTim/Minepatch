/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       mod.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   21.03.25, 12:29
 */
pub(super) mod data;
pub use data::*;
mod error;
pub use error::*;
mod func;
mod msg;

pub use func::*;
pub use msg::*;
