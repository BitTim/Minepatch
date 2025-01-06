/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       forge_based.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   06.01.25, 17:55
 */
use crate::commands::vault::meta::meta_error::MetaError;
use crate::commands::vault::meta::Meta;
use crate::common::error;
use crate::common::error::ErrorType;
use colored::Colorize;
use toml::Table;

pub(crate) fn extract_meta(
    data: &str,
    loader: &str,
    extra: &Option<String>,
) -> error::Result<Meta> {
    let table = data.parse::<Table>()?;
    let jar_version = extract_version_from_extra(extra);

    let meta = meta_from_root_table(&table, loader, &jar_version).ok_or_else(|| {
        MetaError::MalformedMetaFile
            .builder()
            .context("Detected loader", loader)
            .build()
    })?;
    Ok(meta)
}

fn extract_version_from_extra(extra: &Option<String>) -> Option<String> {
    let extra = if let Some(extra) = extra {
        extra
    } else {
        return None;
    };

    let lines = extra.lines();
    for line in lines {
        let mut tokens = line.split(": ");
        let result = loop {
            let token = tokens.next();
            if token.is_none() {
                break None;
            }

            if token.unwrap().trim() == "Implementation-Version" {
                break tokens.next().map(|value| value.to_owned());
            }
        };

        if result.is_some() {
            return result;
        };
    }

    None
}

fn extract_mods_table(root_table: &Table) -> Option<&Table> {
    root_table.get("mods")?.as_array()?.first()?.as_table()
}

fn extract_dep_table<'a>(
    root_table: &'a Table,
    mod_id: &str,
    dependency_id: &str,
) -> Option<&'a Table> {
    let dependencies = root_table
        .get("dependencies")?
        .as_table()?
        .get(mod_id)?
        .as_array()?;
    dependencies
        .iter()
        .filter_map(|dependency| dependency.as_table())
        .find(|dep_table| {
            extract_string(dep_table, "modId").map_or(false, |value| value == dependency_id)
        })
}

fn extract_string(table: &Table, key: &str) -> Option<String> {
    table.get(key)?.as_str().map(ToOwned::to_owned)
}

fn extract_version_range(table: Option<&Table>) -> Option<String> {
    table.map_or(None, |t| extract_string(t, "versionRange"))
}

fn meta_from_root_table(
    root_table: &Table,
    loader: &str,
    jar_version: &Option<String>,
) -> Option<Meta> {
    let mods_table = extract_mods_table(root_table)?;
    let mod_id = extract_string(mods_table, "modId")?;
    let loader_dep_table = extract_dep_table(root_table, &mod_id, &loader.to_lowercase());
    let minecraft_dep_table = extract_dep_table(root_table, &mod_id, "minecraft");

    let authors_value =
        extract_string(mods_table, "authors").or_else(|| extract_string(root_table, "authors"))?;

    let authors = authors_value
        .split(',')
        .map(str::trim)
        .map(ToOwned::to_owned)
        .collect::<Vec<String>>();

    let version = extract_string(mods_table, "version")?;
    let actual_version = if version == "${file.jarVersion}" {
        match jar_version {
            None => "?".red().to_string(),
            Some(version) => version.to_owned(),
        }
    } else {
        version
    };

    Some(Meta {
        id: mod_id,
        name: extract_string(mods_table, "displayName")?,
        version: actual_version,
        description: extract_string(mods_table, "description")?,
        authors,
        loader: loader.to_owned(),
        loader_version: extract_version_range(loader_dep_table),
        minecraft_version: extract_version_range(minecraft_dep_table),
    })
}
