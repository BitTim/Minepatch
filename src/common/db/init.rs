/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       init.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   01.03.25, 00:53
 */
use crate::db::{get_schema_version, set_schema_version};
use crate::{file, prelude};
use rusqlite::Connection;

const DB_FILENAME: &str = "minepatch.db";

fn connect() -> prelude::Result<Connection> {
    let mut path = file::get_data_path()?;
    path.push(DB_FILENAME);

    Ok(Connection::open(path)?)
}

fn create_tables(conn: &Connection) -> prelude::Result<()> {
    conn.execute_batch(
        "
        BEGIN;
        CREATE TABLE IF NOT EXISTS schema_version (version INTEGER NOT NULL);

        CREATE TABLE IF NOT EXISTS mod (
            hash TEXT NOT NULL PRIMARY KEY,
            path TEXT NOT NULL,
            modid TEXT,
            name TEXT,
            version TEXT,
            description TEXT,
            authors TEXT,
            loader TEXT,
            loader_version TEXT,
            mc_version TEXT
        );

        CREATE TABLE IF NOT EXISTS template (
            name TEXT NOT NULL PRIMARY KEY,
            version TEXT,
            loader TEXT,
            download TEXT
        );

        CREATE TABLE IF NOT EXISTS bundle (
            name TEXT NOT NULL PRIMARY KEY,
            description TEXT,
            template TEXT,
            
            FOREIGN KEY (template) REFERENCES template(name)
        );

        CREATE TABLE IF NOT EXISTS patch (
            name TEXT NOT NULL,
            bundle TEXT NOT NULL,
            dependency TEXT,

            PRIMARY KEY (name, bundle),
            FOREIGN KEY (bundle) REFERENCES bundle(name)
        );

        CREATE TABLE IF NOT EXISTS patch_with_mods (
            patch TEXT NOT NULL,
            bundle TEXT NOT NULL,
            mod TEXT NOT NULL,
            removed BOOLEAN NOT NULL,
            
            PRIMARY KEY (patch, bundle, mod),
            FOREIGN KEY (patch, bundle) REFERENCES patch(name, bundle),
            FOREIGN KEY (bundle) REFERENCES bundle(name),
            FOREIGN KEY (mod) REFERENCES mod(hash)
        );

        CREATE TABLE IF NOT EXISTS instance (
            name TEXT NOT NULL PRIMARY KEY,
            path TEXT NOT NULL,
            bundle TEXT NOT NULL,
            patch TEXT NOT NULL,
            
            FOREIGN KEY (bundle) REFERENCES bundle(name),
            FOREIGN KEY (patch, bundle) REFERENCES patch(name, bundle)
        );
        COMMIT;
    ",
    )?;

    if get_schema_version(conn).is_err() {
        set_schema_version(conn, 1)?;
    }

    Ok(())
}

pub fn init() -> prelude::Result<Connection> {
    let conn = connect()?;
    create_tables(&conn)?;

    Ok(conn)
}
