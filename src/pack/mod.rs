/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       mod.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   16.01.25, 17:32
 */
mod cli;
pub use cli::*;
mod data;
pub use data::*;
mod error;
pub use error::*;
mod func;
pub use func::*;
