/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       repo.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   04.02.25, 18:25
 */
use crate::common::Repo;
use crate::instance::data::query::InstanceQuery;
use crate::instance::data::Instance;

pub(crate) struct InstanceRepo {}
impl Repo<InstanceQuery, Instance> for InstanceRepo {}
