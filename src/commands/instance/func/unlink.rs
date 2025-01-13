/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       unlink.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   13.01.25, 21:36
 */
use crate::commands::instance::common::confirm::confirm_unlink;
use crate::commands::instance::common::registry::check_instance;
use crate::commands::instance::data::Instance;
use crate::commands::instance::error::InstanceError;
use crate::common::error::ErrorType;
use crate::common::output::status::{State, StatusOutput};
use crate::common::output::Output;
use crate::common::{error, file};

pub fn unlink(name: &Option<String>, all: &bool, yes: &bool) -> error::Result<()> {
    let mut instances = file::read_all()?;

    let names: Vec<String> = if *all {
        instances
            .iter()
            .map(|instance: &Instance| instance.name.to_owned())
            .collect()
    } else {
        match name {
            Some(name) => vec![name.to_owned()],
            None => return Err(InstanceError::NameNotFound.builder().build()),
        }
    };

    for name in names {
        let (index, instance) = check_instance(&instances, &name, true)?.unwrap();
        if !yes && !confirm_unlink(instance)? {
            StatusOutput::new(State::Abort, "Did not unlink instance")
                .context("Name", &instance.name)
                .print();
            continue;
        }

        instances.remove(index);
        StatusOutput::new(State::Success, "Unlinked instance")
            .context("Name", &*name)
            .print();
    }

    file::write_all(instances)?;
    Ok(())
}
