/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       repo.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   04.02.25, 18:24
 */

use crate::common::Repo;
use crate::pack::data::query::PackQueries;
use crate::pack::Pack;

pub(crate) struct PackRepo {}

impl Repo<PackQueries, Pack> for PackRepo {}
