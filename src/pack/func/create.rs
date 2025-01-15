/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       create.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   15.01.25, 12:05
 */
use crate::pack::data::pack::Pack;
use crate::pack::data::patch::Patch;
use crate::util::output::status::{State, StatusOutput};
use crate::util::output::{format_string_option, Output};
use crate::util::{error, file};
use sha256::Sha256Digest;
use std::path::PathBuf;

pub fn create(
    name: &str,
    from: &Option<String>,
    instance: &Option<String>,
    silent: &bool,
) -> error::Result<()> {
    let mut packs = file::read_all()?;

    match from {
        None => {
            packs.push(Pack::empty(name));
        }
        Some(from) => {
            let mut path = PathBuf::from(from);
            path.push("mods");

            file::check_exists(&*path)?;
            let hashes = file::extract_hashes_from_dir(&*path, *silent)?;

            let state_hash = hashes.join("\n").digest();
            let patch = Patch::new("init", "", &*state_hash, &hashes, &[]);

            packs.push(Pack::new(name, None, &*vec![patch]));

            if let Some(_instance) = instance {
                // TODO: Run 'instance link' command with the specified name and path
            }
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
