/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       mod.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   05.01.25, 18:42
 */
use std::fmt::{Debug, Display};

pub mod list;
pub mod status;
pub mod table;

pub trait Output: Debug + Display {
    fn print(&self) {
        println!("{}", self)
    }
}
