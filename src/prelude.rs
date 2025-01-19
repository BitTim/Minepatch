/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       prelude.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   19.01.25, 12:08
 */

pub use crate::error::Error;
pub type Result<T> = core::result::Result<T, Error>;
pub struct W<T>(pub T);
