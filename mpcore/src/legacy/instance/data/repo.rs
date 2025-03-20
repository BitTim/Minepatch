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
use crate::legacy::instance::data::Instance;
use crate::legacy::instance::data::filter::InstanceFilter;

pub(crate) struct InstanceRepo {}
impl Repo<InstanceFilter, Instance> for InstanceRepo {}
