/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       repo.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   25.03.25, 17:56
 */
use crate::db::Repo;
use crate::patch::data::filter::PatchFilter;
use crate::patch::data::model::Patch;
use crate::prelude::*;
use rusqlite::Connection;
use std::collections::HashSet;

pub struct PatchRepo {}
impl Repo<PatchFilter, Patch> for PatchRepo {}

impl PatchRepo {
    pub fn exists(conn: &Connection, name: &str, bundle: &str) -> Result<bool> {
        PatchRepo::exists_by_filter(
            conn,
            &PatchFilter::build_name_filter(Some(name), Some(bundle), true),
        )
    }

    pub fn by_name(conn: &Connection, name: &str, bundle: &str) -> Result<Patch> {
        PatchRepo::by_filter(
            conn,
            &PatchFilter::build_name_filter(Some(name), Some(bundle), true),
        )
    }

    pub fn by_name_many(
        conn: &Connection,
        name: Option<&str>,
        bundle: Option<&str>,
        exact: bool,
    ) -> Result<HashSet<Patch>> {
        PatchRepo::by_filter_many(conn, &PatchFilter::build_name_filter(name, bundle, exact))
    }

    pub fn by_dependency(conn: &Connection, dependency: &str, bundle: &str) -> Result<Patch> {
        PatchRepo::by_filter(
            conn,
            &PatchFilter::build_dependency_filter(Some(dependency), Some(bundle), true),
        )
    }

    pub fn by_dependency_many(
        conn: &Connection,
        dependency: Option<&str>,
        bundle: Option<&str>,
        exact: bool,
    ) -> Result<HashSet<Patch>> {
        PatchRepo::by_filter_many(
            conn,
            &PatchFilter::build_dependency_filter(dependency, bundle, exact),
        )
    }

    pub fn by_pack(conn: &Connection, bundle: &str) -> Result<Patch> {
        PatchRepo::by_filter(conn, &PatchFilter::build_bundle_filter(Some(bundle), true))
    }

    pub fn by_pack_many(
        conn: &Connection,
        bundle: Option<&str>,
        exact: bool,
    ) -> Result<HashSet<Patch>> {
        PatchRepo::by_filter_many(conn, &PatchFilter::build_bundle_filter(bundle, exact))
    }
}
