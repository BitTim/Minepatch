/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       schema.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   20.03.25, 11:26
 */
use crate::prelude::*;
use rusqlite::{params, Connection};

pub(crate) fn get_schema_version(conn: &Connection) -> Result<i32> {
    Ok(
        conn.query_row("SELECT version FROM schema_version", params![], |row| {
            row.get(0)
        })?,
    )
}

pub(crate) fn set_schema_version(conn: &Connection, version: i32) -> Result<()> {
    conn.execute("UPDATE schema_version SET version = ?", params![version])?;
    Ok(())
}
