/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       repo.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   20.03.25, 11:26
 */

use crate::bundle::data::filter::BundleFilter;
use crate::bundle::Bundle;
use crate::db::Repo;

pub(crate) struct BundleRepo {}

impl Repo<BundleFilter, Bundle> for BundleRepo {}
