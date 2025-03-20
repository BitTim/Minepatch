/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       repo.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   20.03.25, 11:45
 */

use crate::db::Repo;
use crate::legacy::patch::data::filter::PatchFilter;
use crate::legacy::patch::data::model::Patch;

pub(crate) struct PatchRepo {}
impl Repo<PatchFilter, Patch> for PatchRepo {}
