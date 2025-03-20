/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       prelude.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   20.03.25, 11:32
 */

pub use crate::event::Event;
pub use crate::legacy::msg::*;
pub use crate::Error;
pub type Result<T> = core::result::Result<T, Error>;
pub struct W<T>(pub T);
