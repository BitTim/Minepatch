/*
 * Copyright (c) 2024-2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       main.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   16.01.25, 17:33
 */
use clap::Parser;
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use minepatch::instance::cli::InstanceCommands;
use minepatch::pack::PackCommands;
use minepatch::update::func;
use minepatch::util::cli::{Cli, Commands};
use minepatch::util::error;
use minepatch::vault::cli::VaultCommands;
use minepatch::vault::func::{add, list, remove};
use minepatch::{instance, pack};
use std::ffi::OsStr;
use std::process;
use std::thread::sleep;
use std::time::Duration;
use sysinfo::{Pid, Process, ProcessStatus, System};

fn match_command(command: &Commands) -> error::Result<()> {
    match command {
        Commands::Update => func::update::update()?,
        Commands::Instance {
            instance_commands: instance_command,
        } => match instance_command {
            InstanceCommands::List => {
                instance::func::list::list()?;
            }
            InstanceCommands::Link { path, name } => {
                instance::func::link::link(path, name)?;
            }
            InstanceCommands::Rename { name, new_name } => {
                instance::func::rename::rename(name, new_name)?;
            }
            InstanceCommands::Unlink { name, all, yes } => {
                instance::func::unlink::unlink(name, all, yes)?;
            }
        },
        Commands::Vault {
            vault_commands: vault_command,
        } => match vault_command {
            VaultCommands::Add {
                path,
                silent,
                overwrite,
            } => {
                add::add(path, silent, overwrite)?;
            }
            VaultCommands::List { detailed, hash, id } => {
                list::list(detailed, hash, id)?;
            }
            VaultCommands::Remove {
                hash,
                all,
                yes,
                silent,
            } => {
                remove::remove(hash, all, yes, silent)?;
            }
        },
        Commands::Pack { pack_command } => match pack_command {
            PackCommands::List => {}
            PackCommands::Create {
                name,
                from,
                instance,
                silent,
            } => {
                pack::create(name, from, instance, silent)?;
            }
            PackCommands::Delete => {}
        },
    }

    Ok(())
}

fn await_exclusive() -> error::Result<()> {
    let pid = process::id();

    let spinner = ProgressBar::new_spinner();
    spinner.set_style(ProgressStyle::with_template("{spinner} {msg}")?);
    spinner.enable_steady_tick(Duration::from_millis(100));

    loop {
        let sys = System::new_all();
        let processes = sys
            .processes_by_name(OsStr::new("minepatch"))
            .collect::<Vec<&Process>>();

        match &processes.is_empty() {
            true => {
                break;
            }
            false => {
                let mut counter = 0;
                let total = processes.iter().count() - 1;

                for proc in &processes {
                    if proc.pid() == Pid::from_u32(pid) {
                        continue;
                    }

                    if let ProcessStatus::Run = proc.status() {
                        counter += 1;

                        spinner.set_message(format!(
                            "{}\n\t{}{}\n\t{}{}\n\t{}{}s\n\t{}{}",
                            "Minepatch is already running. Waiting for other process to end to prevent data corruption.".bold().purple(),
                            "Name:     ".bold().yellow(),
                            proc.name().to_str().or(Some("")).unwrap().trim(),
                            "PID:      ".bold().yellow(),
                            proc.pid().to_string().trim(),
                            "Time:     ".bold().yellow(),
                            proc.run_time().to_string(),
                            "Conflict: ".bold().yellow(),
                            format!(
                                "{} / {}",
                                counter,
                                total
                            ).bold().cyan()
                        ));

                        sleep(Duration::from_millis(1000));
                    }
                }

                if counter > 0 {
                    continue;
                }
                break;
            }
        }
    }

    Ok(())
}

fn main() {
    if let Err(error) = await_exclusive() {
        println!("{}", error);
    }

    let cli = Cli::parse();

    if let Err(error) = match_command(&cli.command) {
        println!("{}", error);
    }
}
