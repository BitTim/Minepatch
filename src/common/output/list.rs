/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       list.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   05.01.25, 19:25
 */
use crate::common::output::Output;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub struct ListOutput {}

impl Display for ListOutput {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

impl Output for ListOutput {}
