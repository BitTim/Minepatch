/*
 * Copyright (c) 2024-2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       mod.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   27.01.25, 10:20
 */

pub(super) mod data;
pub use data::Instance;
mod error;
pub use error::*;
mod func;
pub use func::*;
