/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       repo.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   06.02.25, 02:18
 */
use crate::db::Repo;
use crate::patch_with_mods::data::filter::PatchModRelFilter;
use crate::patch_with_mods::data::model::PatchWithMods;

pub(crate) struct PatchModRelRepo {}
impl Repo<PatchModRelFilter, PatchWithMods> for PatchModRelRepo {}
