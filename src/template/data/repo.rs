/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       repo.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   17.02.25, 19:28
 */
use crate::db::Repo;
use crate::template::data::Template;
use crate::template::data::filter::TemplateFilter;
pub(crate) struct TemplateRepo {}
impl Repo<TemplateFilter, Template> for TemplateRepo {}
