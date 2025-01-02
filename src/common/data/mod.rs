/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       mod.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   02.01.25, 18:53
 */
use crate::commands::instance::Instance;
use crate::commands::vault::Mod;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;

pub trait DataType: Debug + Serialize + DeserializeOwned {
    const FILENAME: &'static str;
}

impl DataType for Instance {
    const FILENAME: &'static str = "instances.csv";
}
impl DataType for Mod {
    const FILENAME: &'static str = "mods.json";
}
