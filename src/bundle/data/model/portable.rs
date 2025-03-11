/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       portable.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   11.03.25, 05:58
 */
use crate::bundle::Bundle;
use crate::bundle::data::BundleRepo;
use crate::db::{Portable, Repo};
use crate::patch::{Patch, PatchRepo};
use crate::patch_with_mods::{PatchModRelRepo, PatchModRelation};
use crate::prelude::*;
use crate::template::{Template, TemplateFilter, TemplateRepo};
use crate::vault::{ModFilter, PortableMod, VaultRepo};
use crate::{bundle, patch, patch_with_mods, template, vault};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Hash, Debug, Clone, Serialize, Deserialize)]
pub struct PortableBundle {
    pub bundle: Bundle,
    pub template: Option<Template>,
    pub patches: Vec<Patch>,
    pub relations: Vec<PatchModRelation>,
    pub mods: Vec<PortableMod>,
}

impl PortableBundle {
    pub fn new(conn: &Connection, name: &str) -> Result<Self> {
        let bundle = bundle::query_single(conn, name)?;
        let template = bundle
            .template
            .to_owned()
            .map(|template| template::query_single(conn, &template))
            .transpose()?;
        let patches = patch::query_multiple(conn, None, Some(name))?;
        let relations = patch_with_mods::query_multiple_by_bundle(conn, name)?;

        let mut mod_hashes = relations
            .iter()
            .map(|rel| rel.mod_hash.to_owned())
            .collect::<Vec<String>>();
        mod_hashes.dedup();

        let mods = mod_hashes
            .iter()
            .map(|hash| vault::query_single(conn, hash).map(PortableMod::new)?)
            .collect::<Result<Vec<PortableMod>>>()?;

        Ok(Self {
            bundle,
            template,
            patches: Vec::from_iter(patches),
            relations: Vec::from_iter(relations),
            mods,
        })
    }

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

        if let Some(template) = self.template {
            let filter = TemplateFilter::QueryNameExact {
                name: template.name.to_owned(),
            };
            if !TemplateRepo::exists(conn, &filter)? {
                TemplateRepo::insert(conn, template)?;
            }
        }

        for patch in self.patches {
            PatchRepo::insert(conn, patch)?;
        }

        for value in self.mods {
            let filter = ModFilter::QueryHashExact {
                hash: value.hash.to_owned(),
            };
            if !VaultRepo::exists(conn, &filter)? {
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
    fn file_extension() -> String {
        "mpb".to_owned()
    }

    fn object_name(&self) -> String {
        self.bundle.name.to_owned()
    }
}
