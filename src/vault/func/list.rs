/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       list.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   15.01.25, 11:46
 */

use crate::util::output::detailed::{DetailedDisplayObject, DetailedOutput};
use crate::util::output::table::TableOutput;
use crate::util::output::Output;
use crate::util::{error, file};
use crate::vault::data::Mod;
use crate::vault::func::common::registry::filter_registry;
use crate::vault::output::ModDisplay;
use tabled::settings::object::Columns;

pub fn list(detailed: &bool, hash: &Option<String>, id: &Option<String>) -> error::Result<()> {
    let registry: Vec<Mod> = file::read_all()?;
    let filtered = filter_registry(&registry, hash, id);

    let mod_displays = filtered
        .iter()
        .map(|entry| entry.to_display())
        .collect::<Vec<ModDisplay>>();

    match detailed {
        false => {
            TableOutput::new(mod_displays)
                .right(Columns::new(3..4))
                .right(Columns::new(5..6))
                .center(Columns::new(6..7))
                .print();
        }
        true => {
            DetailedOutput::new(
                filtered
                    .iter()
                    .map(|&entry| entry.to_detailed_display())
                    .collect::<Vec<DetailedDisplayObject>>(),
            )
            .print();
        }
    }

    Ok(())
}
