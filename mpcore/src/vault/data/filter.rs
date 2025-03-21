/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       filter.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   21.03.25, 14:22
 */
use crate::common::db::{Entity, Filter, InsertableFilter};
use crate::error::Error;
use crate::hash::Hash;
use crate::vault::VaultError;
use crate::vault::data::Mod;
use rusqlite::ToSql;

pub(crate) enum VaultFilter {
    Insert {
        entry: Box<Mod>,
    },
    ByHashExact {
        hash: Hash,
    },
    ByHashSimilar {
        hash: Hash,
    },
    ByHashAndIDAndNameExact {
        hash: String,
        mod_id: String,
        name: String,
    },
    ByHashAndIDAndNameSimilar {
        hash: String,
        mod_id: String,
        name: String,
    },
}

impl Filter for VaultFilter {
    fn value(&self) -> String {
        match self {
            VaultFilter::Insert { .. } => "VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            VaultFilter::ByHashExact { .. } => "WHERE hash = ?1",
            VaultFilter::ByHashSimilar { .. } => "WHERE hash LIKE ?1||'%'",
            VaultFilter::ByHashAndIDAndNameExact { .. } => {
                "WHERE hash = ?1 AND modid = ?2 AND name = ?3"
            }
            VaultFilter::ByHashAndIDAndNameSimilar { .. } => {
                "WHERE hash LIKE ?1||'%' AND modid LIKE '%'||?2||'%' AND name LIKE '%'||?3||'%'"
            }
        }
        .to_owned()
    }

    fn params(&self) -> Vec<Box<dyn ToSql>> {
        match self {
            VaultFilter::Insert { entry } => entry.to_values(),
            VaultFilter::ByHashExact { hash } | VaultFilter::ByHashSimilar { hash } => {
                vec![Box::new(hash.to_owned())]
            }
            VaultFilter::ByHashAndIDAndNameExact { hash, mod_id, name }
            | VaultFilter::ByHashAndIDAndNameSimilar { hash, mod_id, name } => vec![
                Box::new(hash.to_owned()),
                Box::new(mod_id.to_owned()),
                Box::new(name.to_owned()),
            ],
        }
    }

    fn error(&self) -> Error {
        match self {
            VaultFilter::Insert { entry } => Error::Vault(VaultError::HashTaken {
                hash: entry.hash.to_owned(),
            }),
            VaultFilter::ByHashExact { hash }
            | VaultFilter::ByHashSimilar { hash }
            | VaultFilter::ByHashAndIDAndNameExact { hash, .. }
            | VaultFilter::ByHashAndIDAndNameSimilar { hash, .. } => {
                Error::Vault(VaultError::NotFound {
                    hash: hash.to_owned(),
                })
            }
        }
    }
}

impl InsertableFilter<Mod> for VaultFilter {
    fn insert(value: Mod) -> Self {
        Self::Insert {
            entry: Box::new(value),
        }
    }
}

impl VaultFilter {
    pub(crate) fn build_hash_filter(hash: Option<&Hash>, exact: bool) -> VaultFilter {
        let hash = hash.unwrap_or(&Hash::from("")).to_owned();

        match exact {
            false => VaultFilter::ByHashSimilar { hash },
            true => VaultFilter::ByHashExact { hash },
        }
    }

    pub(crate) fn build_hash_and_id_and_name_filter(
        hash: Option<&Hash>,
        id: Option<&str>,
        name: Option<&str>,
        exact: bool,
    ) -> VaultFilter {
        let hash = hash.unwrap_or(&Hash::from("")).to_owned();
        let id = id.unwrap_or_default().to_owned();
        let name = name.unwrap_or_default().to_owned();

        match exact {
            false => VaultFilter::ByHashAndIDAndNameSimilar {
                hash,
                mod_id: id,
                name,
            },
            true => VaultFilter::ByHashAndIDAndNameExact {
                hash,
                mod_id: id,
                name,
            },
        }
    }
}
