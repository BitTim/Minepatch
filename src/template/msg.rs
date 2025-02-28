/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       msg.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   17.02.25, 19:45
 */
use crate::template::data::Template;
use std::path::PathBuf;
#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub enum TemplateProcess {
    Create,
    Validate,
    Export,
}

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub enum TemplateMessage {
    CreateSuccess { template: Box<Template> },
    ValidateSuccess { name: String },
    ValidateStatus { name: String },
    ExportSuccess { name: String, path: PathBuf },
}
