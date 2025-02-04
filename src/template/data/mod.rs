/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       mod.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   04.02.25, 23:31
 */
mod model;
pub use model::*;
mod query;
pub(crate) use query::*;
mod repo;
pub(super) use repo::*;
