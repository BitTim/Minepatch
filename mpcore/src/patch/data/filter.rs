/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       filter.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   21.03.25, 14:54
 */
use crate::common::db::{Filter, InsertableFilter};
use crate::error::Error;
use crate::patch::{Patch, PatchError};
use rusqlite::ToSql;

pub(crate) enum PatchFilter {
    Insert { patch: Patch },
    ByNameAndBundleExact { name: String, bundle: String },
    ByNameAndBundleSimilar { name: String, bundle: String },
    ByDepAndBundleExact { dependency: String, bundle: String },
    ByDepAndBundleSimilar { dependency: String, bundle: String },
    ByBundleExact { bundle: String },
    ByBundleSimilar { bundle: String },
}

impl Filter for PatchFilter {
    fn value(&self) -> String {
        match self {
            PatchFilter::Insert { .. } => "VALUES (?1, ?2, ?3)",
            PatchFilter::ByNameAndBundleExact { .. } => "WHERE name = ?1 AND bundle = ?2",
            PatchFilter::ByNameAndBundleSimilar { .. } => {
                "WHERE name LIKE ?1||'%' AND bundle LIKE ?2||'%'"
            }
            PatchFilter::ByDepAndBundleExact { .. } => "WHERE dependency = ?1 AND bundle = ?2",
            PatchFilter::ByDepAndBundleSimilar { .. } => {
                "WHERE dependency LIKE ?1||'%' AND bundle LIKE ?2||'%'"
            }
            PatchFilter::ByBundleExact { .. } => "WHERE bundle = ?1",
            PatchFilter::ByBundleSimilar { .. } => "WHERE bundle LIKE ?1||'%",
        }
        .to_owned()
    }

    fn params(&self) -> Vec<Box<dyn ToSql>> {
        match self {
            PatchFilter::Insert { patch } => vec![
                Box::new(patch.name.to_owned()),
                Box::new(patch.bundle.to_owned()),
                Box::new(patch.dependency.to_owned()),
            ],
            PatchFilter::ByNameAndBundleExact { name, bundle }
            | PatchFilter::ByNameAndBundleSimilar { name, bundle } => {
                vec![Box::new(name.to_owned()), Box::new(bundle.to_owned())]
            }
            PatchFilter::ByDepAndBundleExact { dependency, bundle }
            | PatchFilter::ByDepAndBundleSimilar { dependency, bundle } => {
                vec![Box::new(dependency.to_owned()), Box::new(bundle.to_owned())]
            }
            PatchFilter::ByBundleExact { bundle } | PatchFilter::ByBundleSimilar { bundle } => {
                vec![Box::new(bundle.to_owned())]
            }
        }
    }

    fn error(&self) -> Error {
        match self {
            PatchFilter::Insert { patch } => Error::Patch(PatchError::NameExists {
                name: patch.name.to_owned(),
                bundle: patch.bundle.to_owned(),
            }),
            PatchFilter::ByNameAndBundleExact { name, bundle }
            | PatchFilter::ByNameAndBundleSimilar { name, bundle } => {
                Error::Patch(PatchError::NotFound {
                    name: name.to_owned(),
                    bundle: bundle.to_owned(),
                })
            }
            PatchFilter::ByDepAndBundleExact { dependency, bundle }
            | PatchFilter::ByDepAndBundleSimilar { dependency, bundle } => {
                Error::Patch(PatchError::DepNotFound {
                    dependency: dependency.to_owned(),
                    bundle: bundle.to_owned(),
                })
            }
            PatchFilter::ByBundleExact { bundle } | PatchFilter::ByBundleSimilar { bundle } => {
                Error::Patch(PatchError::BundleNotFound {
                    bundle: bundle.to_owned(),
                })
            }
        }
    }
}

impl InsertableFilter<Patch> for PatchFilter {
    fn insert(value: Patch) -> Self {
        Self::Insert { patch: value }
    }
}

impl PatchFilter {
    pub(crate) fn build_name_filter(
        name: Option<&str>,
        bundle: Option<&str>,
        exact: bool,
    ) -> PatchFilter {
        let name = name.unwrap_or_default().to_owned();
        let bundle = bundle.unwrap_or_default().to_owned();

        match exact {
            false => PatchFilter::ByNameAndBundleSimilar { name, bundle },
            true => PatchFilter::ByNameAndBundleExact { name, bundle },
        }
    }

    pub(crate) fn build_dependency_filter(
        dependency: Option<&str>,
        bundle: Option<&str>,
        exact: bool,
    ) -> PatchFilter {
        let dependency = dependency.unwrap_or_default().to_owned();
        let bundle = bundle.unwrap_or_default().to_owned();

        match exact {
            false => PatchFilter::ByDepAndBundleSimilar { dependency, bundle },
            true => PatchFilter::ByDepAndBundleExact { dependency, bundle },
        }
    }

    pub(crate) fn build_bundle_filter(bundle: Option<&str>, exact: bool) -> PatchFilter {
        let bundle = bundle.unwrap_or_default().to_owned();

        match exact {
            false => PatchFilter::ByBundleSimilar { bundle },
            true => PatchFilter::ByBundleExact { bundle },
        }
    }
}
