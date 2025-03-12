/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       mod.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   01.03.25, 00:36
 */
pub(super) mod data;
pub use data::Bundle;
mod error;
pub use error::*;
mod func;
mod msg;

pub use func::*;
pub use msg::*;
