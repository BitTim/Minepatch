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
use crate::patch::data::filter::PatchFilter;
use crate::patch::data::model::Patch;

pub(crate) struct PatchRepo {}
impl Repo<PatchFilter, Patch> for PatchRepo {}
