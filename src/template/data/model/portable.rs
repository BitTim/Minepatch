/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       portable.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   01.03.25, 19:22
 */
use crate::db::Portable;
use crate::template::data::Template;
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Hash, Debug, Clone, Serialize, Deserialize)]
pub struct PortableTemplate {
    template: Template,
}

impl PortableTemplate {
    pub(crate) fn new(template: Template) -> Self {
        Self { template }
    }
}

impl Portable<'_> for PortableTemplate {
    fn file_extension() -> String {
        "mpt".to_owned()
    }

    fn object_name(&self) -> String {
        self.template.name.to_owned()
    }
}
