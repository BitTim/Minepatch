/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       mod.rs
 * Author:     Tim Anhalt (BitTim)
<<<<<<<< HEAD:src/patch_with_mods/mod.rs
 * Modified:   22.01.25, 18:59
 */
mod data;
pub use data::*;
========
 * Modified:   22.01.25, 19:08
 */
pub(crate) mod model;
mod repo;
pub(crate) use repo::*;
>>>>>>>> origin/12-sqlite:src/patch_with_mods/data/mod.rs
