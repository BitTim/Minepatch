/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       create.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   13.01.25, 20:29
 */
use crate::commands::pack::data::pack::Pack;
use crate::commands::pack::data::patch::Patch;
use crate::common::output::status::{State, StatusOutput};
use crate::common::output::{format_string_option, Output};
use crate::common::{error, file};
use sha256::Sha256Digest;
use std::fs;
use std::path::PathBuf;

// TODO: Return some form of "Command runner" as a result
pub fn create(name: &str, from: &Option<String>, instance: &Option<String>) -> error::Result<()> {
    let mut packs = file::read_all()?;

    match from {
        None => {
            packs.push(Pack::empty(name));
        }
        Some(from) => {
            let mut path = PathBuf::from(from);
            path.push("mods");

            file::check_exists(&*path)?;

            let mod_files = fs::read_dir(&path)?;
            let hashes = mod_files
                .map(|mod_file| {
                    let path = mod_file?.path();
                    println!("Found file '{}'", path.display());
                    // TODO: Run 'vault add' command in silent mode for all mod_files (Skip conflicts)

                    // TODO: Retrieve hash from 'vault add' command
                    file::hash_file(&path)
                })
                .filter_map(|result| match result {
                    Ok(hash) => {
                        println!("Generated file hash: '{}'", hash);
                        Some(hash)
                    }
                    Err(error) => {
                        StatusOutput::new(State::Error, "Error occurred during mod file hashing")
                            .context("Message", &*error.to_string())
                            .context("Path", &path.display().to_string())
                            .print();
                        None
                    }
                })
                .collect::<Vec<String>>();

            let state_hash = hashes.join("\n").digest();
            let patch = Patch::new("init", "", &*state_hash, &hashes, &[]);

            packs.push(Pack::new(name, None, &*vec![patch]));

            if let Some(_instance) = instance {
                // TODO: Run 'instance link' command with the specified name and path
            }
        }
    }

    file::write_all(packs)?;
    StatusOutput::new(State::Success, "Created a new pack")
        .context("Name", name)
        .context("From", &*format_string_option(from))
        .print();
    Ok(())
}
