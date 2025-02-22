/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       init.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   11.02.25, 17:39
 */
use crate::{file, prelude};
use rusqlite::Connection;

const DB_FILENAME: &str = "minepatch.db";

fn connect() -> prelude::Result<Connection> {
    let mut path = file::get_data_path()?;
    path.push(DB_FILENAME);

    Ok(Connection::open(path)?)
}

fn create_tables(connection: &Connection) -> prelude::Result<()> {
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

        CREATE TABLE IF NOT EXISTS template (
            name TEXT NOT NULL PRIMARY KEY,
            version TEXT,
            loader TEXT,
            download TEXT
        );

        CREATE TABLE IF NOT EXISTS pack (
            name TEXT NOT NULL PRIMARY KEY,
            description TEXT,
            template TEXT,
            
            FOREIGN KEY (template) REFERENCES template(name)
        );

        CREATE TABLE IF NOT EXISTS patch (
            name TEXT NOT NULL,
            pack TEXT NOT NULL,
            dependency TEXT,

            PRIMARY KEY (name, pack),
            FOREIGN KEY (pack) REFERENCES pack(name)
        );

        CREATE TABLE IF NOT EXISTS patch_with_mods (
            patch TEXT NOT NULL,
            pack TEXT NOT NULL,
            mod TEXT NOT NULL,
            removed BOOLEAN NOT NULL,
            
            PRIMARY KEY (patch, pack, mod),
            FOREIGN KEY (patch, pack) REFERENCES patch(name, pack),
            FOREIGN KEY (pack) REFERENCES pack(name),
            FOREIGN KEY (mod) REFERENCES mod(hash)
        );

        CREATE TABLE IF NOT EXISTS instance (
            name TEXT NOT NULL PRIMARY KEY,
            path TEXT NOT NULL,
            pack TEXT NOT NULL,
            patch TEXT NOT NULL,
            
            FOREIGN KEY (pack) REFERENCES pack(name),
            FOREIGN KEY (patch, pack) REFERENCES patch(name, pack)
        );
        COMMIT;
    ",
    )?)
}

pub fn init() -> prelude::Result<Connection> {
    let connection = connect()?;
    create_tables(&connection)?;

    Ok(connection)
}
