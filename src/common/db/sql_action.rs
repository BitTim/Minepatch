/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       sql_action.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   06.02.25, 02:18
 */
pub(crate) enum SqlAction {
    Insert,
    Select,
    Delete,
}

impl SqlAction {
    pub(crate) fn to_sql(&self) -> String {
        match self {
            SqlAction::Insert => "INSERT INTO",
            SqlAction::Select => "SELECT * FROM",
            SqlAction::Delete => "DELETE FROM",
        }
        .to_owned()
    }
}

pub fn build_statement_sql(action: SqlAction, table: &str, filter: &str) -> String {
    action.to_sql() + " " + table + " " + filter
}
