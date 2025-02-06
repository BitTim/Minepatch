/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       filter.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   06.02.25, 02:18
 */
use crate::common::db::{Entity, Filter, InsertableFilter};
use crate::error::Error;
use crate::patch::PatchError;
use crate::patch_with_mods::PatchWithMods;
use rusqlite::ToSql;

pub(crate) enum PatchModRelFilter {
    Insert {
        relation: PatchWithMods,
    },
    QueryByPatchAndPackExact {
        patch: String,
        pack: String,
    },
    ByPatchAndPackAndModHashExact {
        patch: String,
        pack: String,
        mod_hash: String,
    },
}

impl Filter for PatchModRelFilter {
    fn value(&self) -> String {
        match self {
            PatchModRelFilter::Insert { .. } => "VALUES (?1, ?2, ?3, ?4)",
            PatchModRelFilter::QueryByPatchAndPackExact { .. } => "WHERE patch = ?1 AND pack = ?2",
            PatchModRelFilter::ByPatchAndPackAndModHashExact { .. } => {
                "WHERE patch = ?1 AND pack = ?2 AND mod = ?3"
            }
        }
        .to_owned()
    }

    fn params(&self) -> Vec<Box<dyn ToSql>> {
        match self {
            PatchModRelFilter::Insert { relation } => relation.to_params(),
            PatchModRelFilter::QueryByPatchAndPackExact { patch, pack } => {
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
        }
    }

    fn error(&self) -> Error {
        match self {
            PatchModRelFilter::Insert { relation } => Error::Patch(PatchError::RelTaken(
                relation.patch.to_owned(),
                relation.pack.to_owned(),
                relation.mod_hash.to_owned(),
            )),
            PatchModRelFilter::QueryByPatchAndPackExact { patch, pack } => {
                Error::Patch(PatchError::RelEmpty(patch.to_owned(), pack.to_owned()))
            }
            PatchModRelFilter::ByPatchAndPackAndModHashExact {
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

impl InsertableFilter<PatchWithMods> for PatchModRelFilter {
    fn insert(value: PatchWithMods) -> Self {
        Self::Insert { relation: value }
    }
}
