/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       repo.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   06.02.25, 01:57
 */

use crate::db::Repo;
use crate::pack::data::filter::PackFilter;
use crate::pack::Pack;

pub(crate) struct PackRepo {}

impl Repo<PackFilter, Pack> for PackRepo {}
