/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       msg.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   15.02.25, 01:47
 */
use crate::template::Template;

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub enum TemplateProcess {
    Create,
    Validate,
}

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub enum TemplateMessage {
    CreateSuccess { template: Box<Template> },
    ValidateSuccess { name: String },
    ValidateStatus { name: String },
}
