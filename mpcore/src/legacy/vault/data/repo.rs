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
use crate::legacy::vault::data::Mod;
use crate::legacy::vault::data::filter::ModFilter;

pub(crate) struct VaultRepo {}
impl Repo<ModFilter, Mod> for VaultRepo {}
