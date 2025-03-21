/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       mod.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   21.03.25, 12:16
 */
mod func;
mod msg;

pub(crate) use func::*;
pub use msg::*;

pub type Hash = String;
