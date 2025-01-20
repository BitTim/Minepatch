/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       mod.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   19.01.25, 20:26
 */
use crate::common::file;
use crate::common::file::PathBuilder;
use crate::prelude::*;
use rusqlite::Connection;

const _DB_FILENAME: &str = "minepatch.db";

fn _connect() -> Result<Connection> {
    let path = PathBuilder::new(&file::get_data_path()?)
        .push(_DB_FILENAME)
        .build();

    Ok(Connection::open(path)?)
}

fn _create_tables(connection: &Connection) -> Result<()> {
    Ok(connection.execute_batch(
        "
        BEGIN;
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

        CREATE TABLE IF NOT EXISTS base (
            name TEXT NOT NULL PRIMARY KEY,
            loader TEXT,
            version TEXT,
            download TEXT
        );

        CREATE TABLE IF NOT EXISTS pack (
            name TEXT NOT NULL PRIMARY KEY,
            description TEXT,
            base TEXT,
            latest_patch TEXT NOT NULL,
            
            FOREIGN KEY (base) REFERENCES base(name),
            FOREIGN KEY (latest_patch) REFERENCES patch(name)
        );

        CREATE TABLE IF NOT EXISTS patch (
            name TEXT NOT NULL PRIMARY KEY,
            dependency TEXT,
            state_hash TEXT NOT NULL,
            pack TEXT NOT NULL,
            
            FOREIGN KEY (pack) REFERENCES pack(name)
        );

        CREATE TABLE IF NOT EXISTS patch_with_mods (
            patch TEXT NOT NULL,
            mod TEXT NOT NULL,
            removed BOOLEAN NOT NULL,
            
            PRIMARY KEY (patch, mod),
            FOREIGN KEY (patch) REFERENCES patch(name),
            FOREIGN KEY (mod) REFERENCES mod(hash)
        );

        CREATE TABLE IF NOT EXISTS instance (
            name TEXT NOT NULL PRIMARY KEY,
            path TEXT NOT NULL,
            pack TEXT NOT NULL,
            applied_patch TEXT NOT NULL,
            
            FOREIGN KEY (pack) REFERENCES pack(name),
            FOREIGN KEY (applied_patch) REFERENCES patch(name)
        );
        COMMIT;
    ",
    )?)
}

pub fn init() -> Result<Connection> {
    let connection = _connect()?;
    _create_tables(&connection)?;

    Ok(connection)
}