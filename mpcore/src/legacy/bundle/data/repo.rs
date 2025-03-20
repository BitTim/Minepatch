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
use crate::legacy::bundle::data::filter::BundleFilter;
use crate::legacy::bundle::Bundle;

pub(crate) struct BundleRepo {}

impl Repo<BundleFilter, Bundle> for BundleRepo {}
