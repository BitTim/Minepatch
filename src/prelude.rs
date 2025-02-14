/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       prelude.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   14.02.25, 19:11
 */

pub use crate::common::event::Event;
pub use crate::error::Error;
pub use crate::msg::*;
pub type Result<T> = core::result::Result<T, Error>;
pub struct W<T>(pub T);
