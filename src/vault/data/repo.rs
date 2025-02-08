/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       repo.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   06.02.25, 02:00
 */
use crate::db::Repo;
use crate::vault::data::filter::ModFilter;
use crate::vault::data::model::Mod;

pub(crate) struct VaultRepo {}
impl Repo<ModFilter, Mod> for VaultRepo {}
