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
use crate::instance::data::Instance;
use crate::pack::Pack;
use crate::vault::data::Mod;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;

pub trait DataType: Debug + Serialize + DeserializeOwned + PartialEq {
    const FILENAME: &'static str;
}

impl DataType for Instance {
    const FILENAME: &'static str = "instances.json";
}
impl DataType for Mod {
    const FILENAME: &'static str = "mods.json";
}
impl DataType for Pack {
    const FILENAME: &'static str = "packs.json";
}
