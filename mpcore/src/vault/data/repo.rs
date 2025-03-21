/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       repo.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   21.03.25, 14:33
 */
use crate::db::Repo;
use crate::hash::Hash;
use crate::prelude::*;
use crate::vault::data::Mod;
use crate::vault::data::filter::VaultFilter;
use rusqlite::Connection;
use std::collections::HashSet;

pub struct VaultRepo {}
impl Repo<VaultFilter, Mod> for VaultRepo {}

impl VaultRepo {
    pub fn exists(conn: &Connection, hash: &Hash) -> Result<bool> {
        VaultRepo::exists_by_filter(conn, &VaultFilter::build_hash_filter(Some(hash), true))
    }

    pub fn by_hash(conn: &Connection, hash: &Hash, exact: bool) -> Result<Mod> {
        VaultRepo::by_filter(conn, &VaultFilter::build_hash_filter(Some(hash), exact))
    }

    pub fn by_hash_many(
        conn: &Connection,
        hash: Option<&Hash>,
        exact: bool,
    ) -> Result<HashSet<Mod>> {
        VaultRepo::by_filter_many(conn, &VaultFilter::build_hash_filter(hash, exact))
    }

    pub fn by_hash_and_id_and_name(
        conn: &Connection,
        hash: &Hash,
        id: &str,
        name: &str,
        exact: bool,
    ) -> Result<Mod> {
        VaultRepo::by_filter(
            conn,
            &VaultFilter::build_hash_and_id_and_name_filter(
                Some(hash),
                Some(id),
                Some(name),
                exact,
            ),
        )
    }

    pub fn by_hash_and_id_and_name_many(
        conn: &Connection,
        hash: Option<&Hash>,
        id: Option<&str>,
        name: Option<&str>,
        exact: bool,
    ) -> Result<HashSet<Mod>> {
        VaultRepo::by_filter_many(
            conn,
            &VaultFilter::build_hash_and_id_and_name_filter(hash, id, name, exact),
        )
    }
}
