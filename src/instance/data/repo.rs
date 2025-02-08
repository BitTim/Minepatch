/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       repo.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   06.02.25, 02:18
 */
use crate::db::Repo;
use crate::instance::data::filter::InstanceFilter;
use crate::instance::data::Instance;

pub(crate) struct InstanceRepo {}
impl Repo<InstanceFilter, Instance> for InstanceRepo {}
