/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       filter.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   17.02.25, 19:28
 */
use crate::common::db::{Entity, Filter, InsertableFilter};
use crate::error::Error;
use crate::template::TemplateError;
use crate::template::data::Template;
use rusqlite::ToSql;

pub(crate) enum TemplateFilter {
    Insert { template: Template },
    QueryNameExact { name: String },
    QueryNameSimilar { name: String },
}

impl Filter for TemplateFilter {
    fn value(&self) -> String {
        match self {
            TemplateFilter::Insert { .. } => "VALUES (?1, ?2, ?3, ?4)",
            TemplateFilter::QueryNameExact { .. } => "WHERE name = ?1",
            TemplateFilter::QueryNameSimilar { .. } => "WHERE name LIKE ?1||'%'",
        }
        .to_owned()
    }

    fn params(&self) -> Vec<Box<dyn ToSql>> {
        match self {
            TemplateFilter::Insert { template } => template.to_params(),
            TemplateFilter::QueryNameExact { name } => vec![Box::new(name.to_owned())],
            TemplateFilter::QueryNameSimilar { name } => vec![Box::new(name.to_owned())],
        }
    }

    fn error(&self) -> Error {
        match self {
            TemplateFilter::Insert { template } => {
                Error::Template(TemplateError::NameTaken(template.name.to_owned()))
            }
            TemplateFilter::QueryNameExact { name } | TemplateFilter::QueryNameSimilar { name } => {
                Error::Template(TemplateError::NotFound(name.to_owned()))
            }
        }
    }
}

impl InsertableFilter<Template> for TemplateFilter {
    fn insert(value: Template) -> Self {
        Self::Insert { template: value }
    }
}
