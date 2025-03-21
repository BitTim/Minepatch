/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       update.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   20.03.25, 11:26
 */
use crate::db::Repo;
use crate::instance::data::{InstanceFilter, InstanceRepo};
use crate::instance::Instance;
use crate::prelude::*;
use rusqlite::Connection;

pub fn update_patch(conn: &Connection, name: &str, patch: &str) -> Result<()> {
    let filter = InstanceFilter::ByExactName {
        name: name.to_owned(),
    };
    let instance = InstanceRepo::query_single(conn, &filter)?;

    InstanceRepo::update(
        conn,
        &filter,
        Instance::new(name, &instance.path, &instance.bundle, patch),
    )
}
