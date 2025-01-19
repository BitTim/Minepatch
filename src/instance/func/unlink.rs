/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       unlink.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   19.01.25, 13:17
 */
use crate::instance::data::Instance;
use crate::instance::error::InstanceError;
use crate::instance::func::common::confirm::confirm_unlink;
use crate::instance::func::common::registry::check_instance;
use crate::prelude::*;

pub fn unlink(name: &Option<String>, all: &bool, yes: &bool) -> Result<()> {
    let mut instances = /*file::read_all()?*/ vec![];

    let names: Vec<String> = if *all {
        instances
            .iter()
            .map(|instance: &Instance| instance.name.to_owned())
            .collect()
    } else {
        match name {
            Some(name) => vec![name.to_owned()],
            None => return Err(Error::Instance(InstanceError::NameNotFound(String::new()))),
        }
    };

    for name in names {
        let (index, instance) = check_instance(&instances, &name, true)?.unwrap();
        if !yes && !confirm_unlink(instance)? {
            continue;
        }

        instances.remove(index);
    }

    //file::write_all(instances)?;
    Ok(())
}
