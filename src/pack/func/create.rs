/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       create.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   16.01.25, 17:32
 */
use crate::pack::data::Pack;
use crate::pack::data::Patch;
use crate::pack::error::PackError;
use crate::pack::func::common::check_pack;
use crate::util::error::ErrorType;
use crate::util::output::status::{State, StatusOutput};
use crate::util::output::{format_string_option, Output};
use crate::util::{error, file};
use crate::{instance, vault};
use colored::Colorize;
use indicatif::{ProgressBar, ProgressFinish, ProgressStyle};
use sha256::Sha256Digest;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;

pub fn create(
    name: &str,
    from: &Option<String>,
    instance: &Option<String>,
    silent: &bool,
) -> error::Result<()> {
    let mut packs = file::read_all()?;

    if let Some(_) = check_pack(&packs, name).ok() {
        return Err(PackError::PackExists
            .builder()
            .context("Name", name)
            .build());
    }

    match from {
        None => {
            packs.push(Pack::empty(name));
        }
        Some(from) => {
            let mut path = PathBuf::from(from);
            path.push("mods");

            file::check_exists(&*path)?;
            let hashes = add_mods_from_dir(&*path, *silent)?;

            let state_hash = hashes.join("\n").digest();
            let patch = Patch::new("init", "", &*state_hash, &hashes, &[]);

            packs.push(Pack::new(name, None, &*vec![patch]));

            if let Some(instance) = instance {
                let instance_name = match instance.is_empty() {
                    true => None,
                    false => Some(instance.to_owned()),
                };

                instance::func::link::link(&path, &instance_name)?;
            };
        }
    }

    file::write_all(packs)?;
    if !silent {
        StatusOutput::new(State::Success, "Created a new pack")
            .context("Name", name)
            .context("From", &*format_string_option(from))
            .print();
    }
    Ok(())
}

pub(crate) fn add_mods_from_dir(path: &Path, silent: bool) -> error::Result<Vec<String>> {
    let mod_dir_contents = fs::read_dir(&path)?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .collect::<Vec<PathBuf>>();

    let mod_paths = mod_dir_contents
        .iter()
        .filter_map(|path| {
            if path.extension().and_then(|ext| ext.to_str()) != Some("jar") {
                return None;
            }
            Some(path)
        })
        .collect::<Vec<&PathBuf>>();

    let hashing_bar =
        ProgressBar::new(mod_paths.iter().count() as u64).with_finish(ProgressFinish::AndClear);
    hashing_bar.set_style(ProgressStyle::with_template("{msg}\n{spinner} {wide_bar} {percent:>3} % ({human_pos:>5} / {human_len:5})\nElapsed: {elapsed_precise}\tETA: {eta_precise}")?);
    hashing_bar.enable_steady_tick(Duration::from_millis(100));

    let mut errors = vec![];

    let hashes = mod_paths
        .iter()
        .filter_map(|path| {
            hashing_bar.inc(1);
            match vault::func::add::add(&path, &true, &false) {
                Ok(hash) => {
                    if !silent {
                        hashing_bar.set_message(format!(
                            "File\t'{}'\nHash\t'{}'",
                            path.display(),
                            hash
                        ));
                    }
                    Some(hash)
                }
                Err(error) => {
                    errors.push((path, error));
                    if !silent {
                        hashing_bar.set_message(format!(
                            "File\t'{}'\nHash\t'{}'",
                            path.display(),
                            "ERROR"
                        ));
                    }
                    None
                }
            }
        })
        .collect::<Vec<String>>();

    hashing_bar.finish_and_clear();

    if !silent {
        for (path, error) in &errors {
            println!("{}\n{}\n", path.display().to_string().purple(), error)
        }

        let count = errors.iter().count();
        if count > 0 {
            println!("Failed to add {} mods", count);
        }
    }

    Ok(hashes)
}
