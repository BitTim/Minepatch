/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       filter.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   10.03.25, 10:26
 */
use crate::common::db::{Entity, Filter, InsertableFilter};
use crate::error::Error;
use crate::patch::PatchError;
use crate::patch_with_mods::PatchModRelation;
use crate::vault::VaultError;
use rusqlite::ToSql;

pub(crate) enum PatchModRelFilter {
    Insert {
        relation: PatchModRelation,
    },
    ByBundleExact {
        bundle: String,
    },
    ByPatchAndBundleExact {
        patch: String,
        bundle: String,
    },
    ByPatchAndBundleAndModHashExact {
        patch: String,
        bundle: String,
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
            PatchModRelFilter::ByBundleExact { .. } => "WHERE bundle = ?1",
            PatchModRelFilter::ByPatchAndBundleExact { .. } => "WHERE patch = ?1 AND bundle = ?2",
            PatchModRelFilter::ByPatchAndBundleAndModHashExact { .. } => {
                "WHERE patch = ?1 AND bundle = ?2 AND mod = ?3"
            }
            PatchModRelFilter::ByModHashExact { .. } => "WHERE mod = ?1",
        }
        .to_owned()
    }

    fn params(&self) -> Vec<Box<dyn ToSql>> {
        match self {
            PatchModRelFilter::Insert { relation } => relation.to_params(),
            PatchModRelFilter::ByBundleExact { bundle } => vec![Box::new(bundle.to_owned())],
            PatchModRelFilter::ByPatchAndBundleExact { patch, bundle } => {
                vec![Box::new(patch.to_owned()), Box::new(bundle.to_owned())]
            }
            PatchModRelFilter::ByPatchAndBundleAndModHashExact {
                patch,
                bundle,
                mod_hash,
            } => vec![
                Box::new(patch.to_owned()),
                Box::new(bundle.to_owned()),
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
                bundle: relation.bundle.to_owned(),
                hash: relation.mod_hash.to_owned(),
            }),
            PatchModRelFilter::ByBundleExact { bundle } => {
                Error::Patch(PatchError::BundleNotFound {
                    bundle: bundle.to_owned(),
                })
            }
            PatchModRelFilter::ByPatchAndBundleExact { patch, bundle } => {
                Error::Patch(PatchError::RelEmpty {
                    name: patch.to_owned(),
                    bundle: bundle.to_owned(),
                })
            }
            PatchModRelFilter::ByPatchAndBundleAndModHashExact {
                patch,
                bundle,
                mod_hash,
            } => Error::Patch(PatchError::RelNotFound {
                name: patch.to_owned(),
                bundle: bundle.to_owned(),
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

impl InsertableFilter<PatchModRelation> for PatchModRelFilter {
    fn insert(value: PatchModRelation) -> Self {
        Self::Insert { relation: value }
    }
}
