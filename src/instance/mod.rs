/*
 * Copyright (c) 2024-2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       mod.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   22.01.25, 16:44
 */

pub(crate) mod data;
pub use data::Instance;
mod error;
pub use error::*;
mod func;
pub use func::*;
