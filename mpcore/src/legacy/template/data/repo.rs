/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       repo.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   20.03.25, 11:45
 */
use crate::db::Repo;
use crate::legacy::template::data::Template;
use crate::legacy::template::data::filter::TemplateFilter;
pub(crate) struct TemplateRepo {}
impl Repo<TemplateFilter, Template> for TemplateRepo {}
