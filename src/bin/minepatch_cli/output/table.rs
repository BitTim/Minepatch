/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       table.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   11.02.25, 03:30
 */
use crate::output::Output;
use colored::Colorize;
use std::fmt::{Display, Formatter};
use tabled::grid::records::vec_records::{Text, VecRecords};
use tabled::settings::object::{Object, Rows};
use tabled::settings::{Alignment, Format, Style};
use tabled::{Table, Tabled};

#[derive(Debug)]
pub struct TableOutput {
    table: Table,
    count: usize,
    empty_msg: String,
}

impl TableOutput {
    pub fn new<T: Tabled>(values: Vec<T>, empty_msg: String) -> Self {
        let table = Table::new(&values)
            .with(Style::rounded().remove_horizontals())
            .modify(
                Rows::new(0..1),
                Format::content(|s| format!("\x1b[1;4m{}\x1b[0m", s)),
            )
            .to_owned();

        Self {
            table,
            count: values.len(),
            empty_msg,
        }
    }

    pub fn _center<T: Object<VecRecords<Text<String>>>>(mut self, target: T) -> Self {
        self.table = self.table.modify(target, Alignment::center()).to_owned();
        self
    }

    pub fn _right<T: Object<VecRecords<Text<String>>>>(mut self, target: T) -> Self {
        self.table = self.table.modify(target, Alignment::right()).to_owned();
        self
    }
}

impl Display for TableOutput {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.count < 1 {
            write!(f, "{}", self.empty_msg)
        } else {
            write!(
                f,
                "{}\nNumber of items: {}",
                self.table,
                self.count.to_string().bold().purple()
            )
        }
    }
}

impl Output for TableOutput {}
