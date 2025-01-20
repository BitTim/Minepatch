/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       detailed.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   20.01.25, 02:17
 */
use crate::output::_Output;
use std::fmt::{Debug, Display, Formatter};
use tabled::builder::Builder;
use tabled::settings::object::Columns;
use tabled::settings::style::HorizontalLine;
use tabled::settings::width::Wrap;
use tabled::settings::{Format, Style};

#[derive(Debug)]
pub struct Entry {
    pub title: String,
    pub content: String,
}

impl Entry {
    pub fn new(title: &str, content: &str) -> Self {
        Self {
            title: title.to_owned(),
            content: content.to_owned(),
        }
    }
}

#[derive(Debug)]
pub struct DetailedDisplayObject {
    pub identifiers: Vec<Entry>,
    pub fields: Vec<Entry>,
}

impl DetailedDisplayObject {
    pub fn new(identifiers: Vec<Entry>, fields: Vec<Entry>) -> Self {
        Self {
            identifiers,
            fields,
        }
    }
}

impl Display for DetailedDisplayObject {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut table_builder = Builder::new();
        for identifier in &self.identifiers {
            table_builder.push_record([&identifier.title, &identifier.content]);
        }

        for field in &self.fields {
            table_builder.push_record([&field.title, &field.content]);
        }

        let table = table_builder
            .build()
            .with(Style::rounded().horizontals([(
                self.identifiers.len(),
                HorizontalLine::full('─', '┼', '├', '┤'),
            )]))
            .modify(
                Columns::new(0..1),
                Format::content(|s| format!("\x1b[1;4m{}\x1b[0m", s)),
            )
            .modify(Columns::new(1..), Wrap::new(80).keep_words(true))
            .to_string();

        write!(f, "{}", table)
    }
}

#[derive(Debug)]
pub struct DetailedOutput {
    pub objects: Vec<DetailedDisplayObject>,
}

impl Display for DetailedOutput {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for obj in &self.objects {
            write!(f, "{}\n\n", obj)?;
        }

        write!(f, "Found {} matching entries", self.objects.len())
    }
}

impl DetailedOutput {
    pub fn new(objects: Vec<DetailedDisplayObject>) -> Self {
        Self { objects }
    }
}

impl _Output for DetailedOutput {}
