/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       portable.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   25.03.25, 18:05
 */
use crate::bundle::Bundle;
use crate::bundle::data::BundleRepo;
use crate::db::{Portable, Repo};
use crate::hash::Hash;
use crate::patch::{Patch, PatchRepo};
use crate::patch_with_mods;
use crate::patch_with_mods::{PatchModRelRepo, PatchModRelation};
use crate::prelude::*;
use crate::vault::{PortableMod, VaultFilter, VaultRepo};
use bincode::{Decode, Encode};
use rusqlite::Connection;

/// The portable variant of [Bundle]
///
/// A [Bundle] combined with all relations. Implements [Portable] for import / export functionality
#[derive(Eq, PartialEq, Hash, Debug, Clone, Encode, Decode)]
pub struct PortableBundle {
    pub bundle: Bundle,
    pub patches: Vec<Patch>,
    pub relations: Vec<PatchModRelation>,
    pub mods: Vec<PortableMod>,
}

impl PortableBundle {
    /// Creates a new instance of [PortableBundle].
    ///
    /// Takes a database [Connection] and the name of a [Bundle] and creates a [PortableBundle] object. Fetches all relations by itself.
    pub fn new(conn: &Connection, name: &str) -> Result<Self> {
        let bundle = BundleRepo::by_name(conn, name)?;
        let patches = PatchRepo::by_name_many(conn, None, Some(name), true)?;
        let relations = patch_with_mods::query_multiple_by_bundle(conn, name)?;

        let mut mod_hashes = relations
            .iter()
            .map(|rel| rel.mod_hash.to_owned())
            .collect::<Vec<Hash>>();
        mod_hashes.dedup();

        let mods = mod_hashes
            .iter()
            .map(|hash| VaultRepo::by_hash(conn, hash).map(PortableMod::new)?)
            .collect::<Result<Vec<PortableMod>>>()?;

        Ok(Self {
            bundle,
            patches: Vec::from_iter(patches),
            relations: Vec::from_iter(relations),
            mods,
        })
    }

    /// Inserts a [PortableBundle] instance into the database
    ///
    /// Takes a database [Connection] and inserts all relations and the [Bundle] itself into the database. Optionally takes a name that overrides the name set in the instance.
    pub fn insert(mut self, conn: &Connection, name: Option<&str>) -> Result<()> {
        if let Some(name) = name {
            self.bundle.name = name.to_owned();
            for patch in &mut self.patches {
                patch.bundle = name.to_owned();
            }
            for rel in &mut self.relations {
                rel.bundle = name.to_owned();
            }
        }

        BundleRepo::insert(conn, self.bundle)?;

        for patch in self.patches {
            PatchRepo::insert(conn, patch)?;
        }

        for value in self.mods {
            let filter = VaultFilter::ByHashExact {
                hash: value.hash.to_owned(),
            };
            if !VaultRepo::exists_by_filter(conn, &filter)? {
                value.insert(conn)?;
            }
        }

        for rel in self.relations {
            PatchModRelRepo::insert(conn, rel)?;
        }

        Ok(())
    }
}

impl Portable for PortableBundle {
    /// Returns the file extension to use for exported [PortableBundles].
    fn file_extension() -> String {
        "mpb".to_owned()
    }

    /// Returns the name for the [PortableBundle] object.
    fn name(&self) -> String {
        self.bundle.name.to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file_extension() {
        assert_eq!(PortableBundle::file_extension(), "mpb");
    }

    #[test]
    fn name() {
        //TODO: Needs Database Mock
        //let pb = PortableBundle::new()
        //assert_eq!(PortableBundle::file_extension(), "mpb");
    }
}
