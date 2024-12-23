/*
 * Copyright (c) 2024 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       main.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   23.12.24, 20:39
 */

use std::env;

fn main() {
    let raw_args: Vec<String> = env::args().collect();

    match parse_args(&raw_args) {
        Ok((cmd, args)) => println!("Command: {} | Args: {:?}", cmd, args),
        Err(e) => println!("{}", e),
    }
}

fn parse_args(raw_args: &[String]) -> Result<(&String, &[String]), &'static str> {
    let cmd = raw_args.get(1).ok_or("No command specified")?;
    let args = raw_args.get(2..).unwrap_or_default();

    Ok((cmd, args))
}
