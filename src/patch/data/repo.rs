/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       repo.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   04.02.25, 19:14
 */

use crate::common::Repo;
use crate::patch::data::model::Patch;
use crate::patch::data::query::PatchQueries;

pub(crate) struct PatchRepo {}
impl Repo<PatchQueries, Patch> for PatchRepo {}
