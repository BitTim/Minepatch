/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       repo.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   04.02.25, 23:21
 */
use crate::common::Repo;
use crate::vault::data::model::Mod;
use crate::vault::data::query::VaultQueries;

pub(crate) struct VaultRepo {}
impl Repo<VaultQueries, Mod> for VaultRepo {}
