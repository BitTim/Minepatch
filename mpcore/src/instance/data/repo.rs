/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       repo.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   25.03.25, 17:54
 */
use crate::db::Repo;
use crate::instance::data::Instance;
use crate::instance::data::filter::InstanceFilter;
use crate::prelude::*;
use rusqlite::Connection;
use std::collections::HashSet;

pub struct InstanceRepo {}
impl Repo<InstanceFilter, Instance> for InstanceRepo {}

impl InstanceRepo {
    pub fn exists(conn: &Connection, name: &str) -> Result<bool> {
        InstanceRepo::exists_by_filter(conn, &InstanceFilter::build_name_filter(Some(name), true))
    }

    pub fn by_name(conn: &Connection, name: &str) -> Result<Instance> {
        InstanceRepo::by_filter(conn, &InstanceFilter::build_name_filter(Some(name), true))
    }

    pub fn by_name_many(
        conn: &Connection,
        name: Option<&str>,
        exact: bool,
    ) -> Result<HashSet<Instance>> {
        InstanceRepo::by_filter_many(conn, &InstanceFilter::build_name_filter(name, exact))
    }

    pub fn by_patch(conn: &Connection, patch: &str) -> Result<Instance> {
        InstanceRepo::by_filter(conn, &InstanceFilter::build_patch_filter(Some(patch), true))
    }

    pub fn by_patch_many(conn: &Connection, patch: &str, exact: bool) -> Result<HashSet<Instance>> {
        InstanceRepo::by_filter_many(
            conn,
            &InstanceFilter::build_patch_filter(Some(patch), exact),
        )
    }
}
