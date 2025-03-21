/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       repo.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   20.03.25, 11:26
 */
use crate::db::Repo;
use crate::template::data::filter::TemplateFilter;
use crate::template::data::Template;
pub(crate) struct TemplateRepo {}
impl Repo<TemplateFilter, Template> for TemplateRepo {}
