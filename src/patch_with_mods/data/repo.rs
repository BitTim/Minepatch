/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       repo.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   04.02.25, 21:44
 */
use crate::common::Repo;
use crate::patch_with_mods::data::model::PatchWithMods;
use crate::patch_with_mods::data::query::PatchModRelQueries;

pub(crate) struct PatchModRelRepo {}
impl Repo<PatchModRelQueries, PatchWithMods> for PatchModRelRepo {}
