/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       msg.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   01.03.25, 19:21
 */
use crate::template::data::Template;
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
