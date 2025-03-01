/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       filter.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   01.03.25, 19:26
 */
use crate::common::db::{Entity, Filter, InsertableFilter};
use crate::error::Error;
use crate::vault::VaultError;
use crate::vault::data::Mod;
use rusqlite::ToSql;

pub(crate) enum ModFilter {
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

impl Filter for ModFilter {
    fn value(&self) -> String {
        match self {
            ModFilter::Insert { .. } => "VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            ModFilter::QueryAll => "",
            ModFilter::QueryHashExact { .. } => "WHERE hash = ?1",
            ModFilter::QueryHashAndIDAndNameSimilar { .. } => {
                "WHERE hash LIKE ?1||'%' AND modid LIKE '%'||?2||'%' AND name LIKE '%'||?3||'%'"
            }
        }
        .to_owned()
    }

    fn params(&self) -> Vec<Box<dyn ToSql>> {
        match self {
            ModFilter::Insert { entry } => entry.to_params(),
            ModFilter::QueryAll => vec![],
            ModFilter::QueryHashExact { hash } => vec![Box::new(hash.to_owned())],
            ModFilter::QueryHashAndIDAndNameSimilar { hash, mod_id, name } => vec![
                Box::new(hash.to_owned()),
                Box::new(mod_id.to_owned()),
                Box::new(name.to_owned()),
            ],
        }
    }

    fn error(&self) -> Error {
        match self {
            ModFilter::Insert { entry } => Error::Vault(VaultError::HashTaken {
                hash: entry.hash.to_owned(),
            }),
            ModFilter::QueryAll => Error::Generic("Error during vault query".to_owned()),
            ModFilter::QueryHashExact { hash }
            | ModFilter::QueryHashAndIDAndNameSimilar { hash, .. } => {
                Error::Vault(VaultError::NotFound {
                    hash: hash.to_owned(),
                })
            }
        }
    }
}

impl InsertableFilter<Mod> for ModFilter {
    fn insert(value: Mod) -> Self {
        Self::Insert {
            entry: Box::new(value),
        }
    }
}
