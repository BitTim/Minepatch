/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       query.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   04.02.25, 23:26
 */
use crate::common::{Query, QueryInsert, QueryModel};
use crate::error::Error;
use crate::vault::{Mod, VaultError};
use rusqlite::ToSql;

pub(crate) enum VaultQueries {
    Insert {
        entry: Box<Mod>,
    },
    QueryAll,
    QueryHashExact {
        hash: String,
    },
    QueryHashAndIDAndNameSimilar {
        hash: String,
        mod_id: String,
        name: String,
    },
}

impl Query for VaultQueries {
    fn value(&self) -> String {
        match self {
            VaultQueries::Insert { .. } => "INSERT INTO mod (hash, path, modid, name, version, description, authors, loader, loader_version, mc_version) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            VaultQueries::QueryAll => "SELECT * FROM mod",
            VaultQueries::QueryHashExact { .. } => "SELECT * FROM mod WHERE hash = ?1",
            VaultQueries::QueryHashAndIDAndNameSimilar { .. } => "SELECT * FROM mod WHERE hash LIKE ?1||'%' AND modid LIKE '%'||?2||'%' AND name LIKE '%'||?3||'%'",
        }.to_owned()
    }

    fn params(&self) -> Vec<Box<dyn ToSql>> {
        match self {
            VaultQueries::Insert { entry } => entry.to_params(),
            VaultQueries::QueryAll => vec![],
            VaultQueries::QueryHashExact { hash } => vec![Box::new(hash.to_owned())],
            VaultQueries::QueryHashAndIDAndNameSimilar { hash, mod_id, name } => vec![
                Box::new(hash.to_owned()),
                Box::new(mod_id.to_owned()),
                Box::new(name.to_owned()),
            ],
        }
    }

    fn error(&self) -> Error {
        match self {
            VaultQueries::Insert { entry } => {
                Error::Vault(VaultError::HashTaken(entry.hash.to_owned()))
            }
            VaultQueries::QueryAll => Error::Vault(VaultError::NotFound("".to_owned())),
            VaultQueries::QueryHashExact { hash }
            | VaultQueries::QueryHashAndIDAndNameSimilar { hash, .. } => {
                Error::Vault(VaultError::NotFound(hash.to_owned()))
            }
        }
    }
}

impl QueryInsert<Mod> for VaultQueries {
    fn insert(value: Mod) -> Self {
        Self::Insert {
            entry: Box::new(value),
        }
    }
}
