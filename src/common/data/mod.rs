/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       mod.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   13.01.25, 21:29
 */
use crate::commands::instance::data::Instance;
use crate::commands::pack::data::pack::Pack;
use crate::commands::vault::Mod;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;

pub trait DataType: Debug + Serialize + DeserializeOwned {
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
