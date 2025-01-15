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
use crate::instance::data::Instance;
use crate::instance::output::InstanceDisplay;
use crate::util::output::table::TableOutput;
use crate::util::output::Output;
use crate::util::{error, file};
use tabled::settings::object::Columns;

pub fn list() -> error::Result<()> {
    let instances: Vec<Instance> = file::read_all()?;
    let instance_displays = instances
        .iter()
        .map(|instance| instance.to_display())
        .collect::<Vec<InstanceDisplay>>();

    TableOutput::new(instance_displays)
        .center(Columns::new(2..3))
        .print();
    Ok(())
}
