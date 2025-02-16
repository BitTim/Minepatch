/*
 * Copyright (c) 2024-2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       mod.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   11.02.25, 02:57
 */

pub(super) mod data;
pub use data::Instance;
mod error;
pub use error::*;
mod func;
mod msg;

pub use func::*;
pub use msg::*;
