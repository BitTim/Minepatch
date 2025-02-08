/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       repo.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   06.02.25, 01:59
 */
use crate::db::Repo;
use crate::template::data::filter::TemplateFilter;
use crate::template::data::model::Template;

pub(crate) struct TemplateRepo {}
impl Repo<TemplateFilter, Template> for TemplateRepo {}
