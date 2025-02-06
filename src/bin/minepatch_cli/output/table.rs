/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       table.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   05.02.25, 21:12
 */
use crate::output::Output;
use minepatch::msg::Message;
use std::fmt::{Display, Formatter};
use tabled::grid::records::vec_records::{Text, VecRecords};
use tabled::settings::object::{Object, Rows};
use tabled::settings::{Alignment, Format, Style};
use tabled::{Table, Tabled};

#[derive(Debug)]
pub struct TableOutput {
    table: Table,
    empty: bool,
    empty_msg: Message,
}

impl TableOutput {
    pub fn new<T: Tabled>(values: Vec<T>, empty_msg: Message) -> Self {
        let table = Table::new(&values)
            .with(Style::rounded().remove_horizontals())
            .modify(
                Rows::new(0..1),
                Format::content(|s| format!("\x1b[1;4m{}\x1b[0m", s)),
            )
            .to_owned();

        Self {
            table,
            empty: values.is_empty(),
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
        if self.empty {
            write!(f, "{}", self.empty_msg)
        } else {
            write!(f, "{}", self.table)
        }
    }
}

impl Output for TableOutput {}
