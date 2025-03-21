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
use crate::db::Repo;
use crate::vault::data::filter::ModFilter;
use crate::vault::data::Mod;

pub(crate) struct VaultRepo {}
impl Repo<ModFilter, Mod> for VaultRepo {}
