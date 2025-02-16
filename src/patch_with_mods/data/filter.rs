/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       filter.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   16.02.25, 13:55
 */
use crate::common::db::{Entity, Filter, InsertableFilter};
use crate::error::Error;
use crate::patch::PatchError;
use crate::patch_with_mods::PatchWithMods;
use crate::vault::VaultError;
use rusqlite::ToSql;

pub(crate) enum PatchModRelFilter {
    Insert {
        relation: PatchWithMods,
    },
    ByPatchAndPackExact {
        patch: String,
        pack: String,
    },
    ByPatchAndPackAndModHashExact {
        patch: String,
        pack: String,
        mod_hash: String,
    },
    ByModHashExact {
        hash: String,
    },
}

impl Filter for PatchModRelFilter {
    fn value(&self) -> String {
        match self {
            PatchModRelFilter::Insert { .. } => "VALUES (?1, ?2, ?3, ?4)",
            PatchModRelFilter::ByPatchAndPackExact { .. } => "WHERE patch = ?1 AND pack = ?2",
            PatchModRelFilter::ByPatchAndPackAndModHashExact { .. } => {
                "WHERE patch = ?1 AND pack = ?2 AND mod = ?3"
            }
            PatchModRelFilter::ByModHashExact { .. } => "WHERE mod = ?1",
        }
        .to_owned()
    }

    fn params(&self) -> Vec<Box<dyn ToSql>> {
        match self {
            PatchModRelFilter::Insert { relation } => relation.to_params(),
            PatchModRelFilter::ByPatchAndPackExact { patch, pack } => {
                vec![Box::new(patch.to_owned()), Box::new(pack.to_owned())]
            }
            PatchModRelFilter::ByPatchAndPackAndModHashExact {
                patch,
                pack,
                mod_hash,
            } => vec![
                Box::new(patch.to_owned()),
                Box::new(pack.to_owned()),
                Box::new(mod_hash.to_owned()),
            ],
            PatchModRelFilter::ByModHashExact { hash: mod_hash } => {
                vec![Box::new(mod_hash.to_owned())]
            }
        }
    }

    fn error(&self) -> Error {
        match self {
            PatchModRelFilter::Insert { relation } => Error::Patch(PatchError::RelTaken {
                name: relation.patch.to_owned(),
                pack: relation.pack.to_owned(),
                hash: relation.mod_hash.to_owned(),
            }),
            PatchModRelFilter::ByPatchAndPackExact { patch, pack } => {
                Error::Patch(PatchError::RelEmpty {
                    name: patch.to_owned(),
                    pack: pack.to_owned(),
                })
            }
            PatchModRelFilter::ByPatchAndPackAndModHashExact {
                patch,
                pack,
                mod_hash,
            } => Error::Patch(PatchError::RelNotFound {
                name: patch.to_owned(),
                pack: pack.to_owned(),
                hash: mod_hash.to_owned(),
            }),
            PatchModRelFilter::ByModHashExact { hash: mod_hash } => {
                Error::Vault(VaultError::RelNotFound {
                    hash: mod_hash.to_owned(),
                })
            }
        }
    }
}

impl InsertableFilter<PatchWithMods> for PatchModRelFilter {
    fn insert(value: PatchWithMods) -> Self {
        Self::Insert { relation: value }
    }
}
