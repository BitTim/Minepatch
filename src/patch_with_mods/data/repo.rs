/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       repo.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   10.03.25, 10:26
 */
use crate::db::Repo;
use crate::patch_with_mods::data::filter::PatchModRelFilter;
use crate::patch_with_mods::data::model::PatchModRelation;

pub(crate) struct PatchModRelRepo {}
impl Repo<PatchModRelFilter, PatchModRelation> for PatchModRelRepo {}
