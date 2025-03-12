/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       repo.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   01.03.25, 19:26
 */
use crate::db::Repo;
use crate::vault::data::Mod;
use crate::vault::data::filter::ModFilter;

pub(crate) struct VaultRepo {}
impl Repo<ModFilter, Mod> for VaultRepo {}
