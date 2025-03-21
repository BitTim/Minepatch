/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       validate.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   21.03.25, 14:48
 */
use crate::db::Repo;
use crate::prelude::*;
use crate::vault::VaultError;
use crate::vault::data::{VaultFilter, VaultRepo};
use rusqlite::Connection;
use std::fs;

pub fn validate(conn: &Connection, hash: &str) -> Result<()> {
    let query = VaultFilter::ByHashExact {
        hash: hash.to_owned(),
    };
    let value = VaultRepo::by_filter(conn, &query)?;

    if !fs::exists(&value.path)? {
        return Err(Error::Vault(VaultError::PathNotExist {
            hash: hash.to_owned(),
            path: value.path.display().to_string(),
        }));
    }

    Ok(())
}
