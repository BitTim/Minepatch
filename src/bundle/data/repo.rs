/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       repo.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   01.03.25, 00:36
 */

use crate::bundle::Bundle;
use crate::bundle::data::filter::BundleFilter;
use crate::db::Repo;

pub(crate) struct BundleRepo {}

impl Repo<BundleFilter, Bundle> for BundleRepo {}
