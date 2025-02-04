/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       repo.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   04.02.25, 22:33
 */
use crate::common::Repo;
use crate::template::data::model::Template;
use crate::template::data::query::TemplateQueries;

pub(crate) struct TemplateRepo {}
impl Repo<TemplateQueries, Template> for TemplateRepo {}
