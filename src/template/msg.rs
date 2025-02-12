/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       msg.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   12.02.25, 17:22
 */
use crate::template::Template;

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum TemplateProcess {
    Create,
    Validate,
}

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum TemplateMessage {
    CreateSuccess { template: Box<Template> },
    ValidateSuccess { name: String },
}
