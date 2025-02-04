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
use crate::template::{Template, TemplateError};
use rusqlite::ToSql;

pub(crate) enum TemplateQueries {
    Insert { template: Template },
    QueryNameExact { name: String },
    QueryNameSimilar { name: String },
}

impl Query for TemplateQueries {
    fn value(&self) -> String {
        match self {
            TemplateQueries::Insert { .. } => "",
            TemplateQueries::QueryNameExact { .. } => "",
            TemplateQueries::QueryNameSimilar { .. } => "",
        }
        .to_owned()
    }

    fn params(&self) -> Vec<Box<dyn ToSql>> {
        match self {
            TemplateQueries::Insert { template } => template.to_params(),
            TemplateQueries::QueryNameExact { name } => vec![Box::new(name.to_owned())],
            TemplateQueries::QueryNameSimilar { name } => vec![Box::new(name.to_owned())],
        }
    }

    fn error(&self) -> Error {
        match self {
            TemplateQueries::Insert { template } => {
                Error::Template(TemplateError::NameTaken(template.name.to_owned()))
            }
            TemplateQueries::QueryNameExact { name }
            | TemplateQueries::QueryNameSimilar { name } => {
                Error::Template(TemplateError::NotFound(name.to_owned()))
            }
        }
    }
}

impl QueryInsert<Template> for TemplateQueries {
    fn insert(value: Template) -> Self {
        Self::Insert { template: value }
    }
}
