/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       query.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   04.02.25, 23:31
 */
use crate::common::{Query, QueryInsert, QueryModel};
use crate::error::Error;
use crate::patch::PatchError;
use crate::patch_with_mods::PatchWithMods;
use rusqlite::ToSql;

pub(crate) enum PatchModRelQueries {
    Insert {
        relation: PatchWithMods,
    },
    QueryByPatchAndPackExact {
        patch: String,
        pack: String,
    },
    QueryByPatchAndPackAndModHashExact {
        patch: String,
        pack: String,
        mod_hash: String,
    },
}

impl Query for PatchModRelQueries {
    fn value(&self) -> String {
        match self {
            PatchModRelQueries::Insert { .. } => {
                "INSERT INTO patch_with_mods (patch, pack, mod, removed) VALUES (?1, ?2, ?3, ?4)"
            }
            PatchModRelQueries::QueryByPatchAndPackExact { .. } => {
                "SELECT patch, pack, mod, removed FROM patch_with_mods WHERE patch = ?1 AND pack = ?2"
            }
            PatchModRelQueries::QueryByPatchAndPackAndModHashExact { .. } => {
                "SELECT patch, pack, mod, removed FROM patch_with_mods WHERE patch = ?1 AND pack = ?2 AND mod = ?3"
            }
        }
            .to_owned()
    }

    fn params(&self) -> Vec<Box<dyn ToSql>> {
        match self {
            PatchModRelQueries::Insert { relation } => relation.to_params(),
            PatchModRelQueries::QueryByPatchAndPackExact { patch, pack } => {
                vec![Box::new(patch.to_owned()), Box::new(pack.to_owned())]
            }
            PatchModRelQueries::QueryByPatchAndPackAndModHashExact {
                patch,
                pack,
                mod_hash,
            } => vec![
                Box::new(patch.to_owned()),
                Box::new(pack.to_owned()),
                Box::new(mod_hash.to_owned()),
            ],
        }
    }

    fn error(&self) -> Error {
        match self {
            PatchModRelQueries::Insert { relation } => {
                Error::Patch(PatchError::ModAlreadyIncluded(
                    relation.patch.to_owned(),
                    relation.pack.to_owned(),
                    relation.mod_hash.to_owned(),
                ))
            }
            PatchModRelQueries::QueryByPatchAndPackExact { patch, pack } => {
                Error::Patch(PatchError::RelEmpty(patch.to_owned(), pack.to_owned()))
            }
            PatchModRelQueries::QueryByPatchAndPackAndModHashExact {
                patch,
                pack,
                mod_hash,
            } => Error::Patch(PatchError::RelNotFound(
                patch.to_owned(),
                pack.to_owned(),
                mod_hash.to_owned(),
            )),
        }
    }
}

impl QueryInsert<PatchWithMods> for PatchModRelQueries {
    fn insert(value: PatchWithMods) -> Self {
        Self::Insert { relation: value }
    }
}
