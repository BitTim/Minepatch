/*
 * Copyright (c) 2024 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       main.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   25.12.24, 01:55
 */
mod args;

use args::Cli;
use clap::Parser;

fn main() {
    let _cli = Cli::parse();
}
