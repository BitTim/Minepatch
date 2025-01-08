/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       table.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   08.01.25, 15:59
 */
use crate::common::output::Output;
use std::fmt::{Display, Formatter};
use tabled::grid::records::vec_records::{Text, VecRecords};
use tabled::settings::object::{Object, Rows};
use tabled::settings::{Alignment, Format, Style};
use tabled::{Table, Tabled};

#[derive(Debug)]
pub struct TableOutput {
    table: Table,
}

impl TableOutput {
    pub(crate) fn new<T: Tabled>(values: Vec<T>) -> Self {
        let table = Table::new(values)
            .with(Style::rounded().remove_horizontals())
            .modify(
                Rows::new(0..1),
                Format::content(|s| format!("\x1b[1;4m{}\x1b[0m", s)),
            )
            .to_owned();

        Self { table }
    }

    pub(crate) fn center<T: Object<VecRecords<Text<String>>>>(mut self, target: T) -> Self {
        self.table = self.table.modify(target, Alignment::center()).to_owned();
        self
    }

    pub(crate) fn right<T: Object<VecRecords<Text<String>>>>(mut self, target: T) -> Self {
        self.table = self.table.modify(target, Alignment::right()).to_owned();
        self
    }
}

impl Display for TableOutput {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.table)
    }
}

impl Output for TableOutput {}
