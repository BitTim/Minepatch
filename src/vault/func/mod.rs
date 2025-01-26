/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       mod.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   26.01.25, 17:37
 */
mod add;
pub use add::*;
mod common;
mod query;
pub use query::*;
mod remove;
pub use remove::*;
mod validate;
pub use validate::*;
