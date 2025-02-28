/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       template.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   17.02.25, 19:29
 */
use crate::output::format_string_option;
use minepatch::template::Template;
use tabled::Tabled;

#[derive(Debug, Tabled)]
pub struct TemplateListItem {
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "Version")]
    version: String,
    #[tabled(rename = "Loader")]
    loader: String,
    #[tabled(rename = "Download")]
    download: String,
}

impl From<&Template> for TemplateListItem {
    fn from(template: &Template) -> Self {
        Self {
            name: template.name.to_owned(),
            version: format_string_option(&template.version),
            loader: format_string_option(&template.loader),
            download: format_string_option(&template.download),
        }
    }
}
