/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       mod.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   02.02.25, 19:25
 */
pub mod db;
pub mod file;
pub mod hash;
pub mod meta;
pub mod msg;
mod query;
pub(crate) use query::Query;
