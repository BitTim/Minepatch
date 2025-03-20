/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       msg.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   20.03.25, 11:30
 */
use crate::legacy::template::data::Template;
use std::path::PathBuf;
#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub enum TemplateProcess {
    Create,
    Validate,
    Export,
    Import,
}

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub enum TemplateMessage {
    CreateSuccess { template: Box<Template> },
    ValidateSuccess { name: String },
    ValidateStatus { name: String },
    ExportSuccess { name: String, path: PathBuf },
    ImportSuccess { name: String, path: PathBuf },
}
