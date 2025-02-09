/*
 * Copyright (c) 2024-2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       main.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   09.02.25, 18:57
 */
use crate::cli::instance::InstanceCommands;
use crate::cli::pack::PackCommands;
use crate::cli::patch::PatchCommands;
use crate::cli::template::TemplateCommands;
use crate::cli::{instance, pack, patch, template, vault, Cli, Commands};
use crate::output::status::{Status, StatusOutput};
use crate::output::Output;
use clap::Parser;
use cli::vault::VaultCommands;
use colored::Colorize;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use minepatch::db;
use minepatch::event::Event;
use minepatch::msg::Message;
use minepatch::prelude::*;
use minepatch::update::func;
use rusqlite::Connection;
use std::collections::HashMap;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

mod cli;
mod output;

fn match_command(command: &Commands, connection: &Connection, tx: &Sender<Event>) -> Result<()> {
    match command {
        Commands::Update => func::update::update()?,
        Commands::Instance {
            instance_commands: instance_command,
        } => match instance_command {
            InstanceCommands::Apply { name, patch } => instance::apply(connection, name, patch)?,
            InstanceCommands::List { name } => instance::list(connection, name)?,
            InstanceCommands::Link { path, name, pack } => {
                instance::link(connection, path, name, pack)?;
            }
        },
        Commands::Vault {
            vault_commands: vault_command,
        } => match vault_command {
            VaultCommands::Add { path, overwrite } => {
                vault::add(connection, tx, path, overwrite)?;
            }
            VaultCommands::List {
                detailed,
                hash,
                id,
                name,
            } => {
                vault::list(connection, detailed, hash, id, name)?;
            }
            VaultCommands::Remove { hash, all, yes } => {
                vault::remove(connection, hash, *all, *yes)?;
            }
        },
        Commands::Template {
            template_commands: template_command,
        } => match template_command {
            TemplateCommands::Create {
                name,
                version,
                loader,
                download,
            } => template::create(connection, name, version, loader, download)?,
            TemplateCommands::List { name } => template::list(connection, name)?,
        },
        Commands::Patch {
            patch_commands: patch_command,
        } => match patch_command {
            PatchCommands::Create {
                name,
                dependency,
                pack,
            } => patch::create(connection, name, pack, dependency)?,
            PatchCommands::Exclude {
                name,
                pack,
                mod_hash,
            } => patch::exclude(connection, name, pack, mod_hash)?,
            PatchCommands::Generate { name, instance } => {
                patch::generate(connection, tx, name, instance)?
            }
            PatchCommands::Include {
                name,
                pack,
                mod_hash,
            } => patch::include(connection, name, pack, mod_hash)?,
            PatchCommands::List { name, pack } => patch::list(connection, name, pack)?,
            PatchCommands::Simulate {
                name,
                pack,
                dir_hash,
            } => patch::simulate(connection, name, pack, dir_hash)?,
            PatchCommands::View { name, pack } => patch::view(connection, name, pack)?,
        },
        Commands::Pack {
            pack_commands: pack_command,
        } => match pack_command {
            PackCommands::List { name } => pack::list(connection, name)?,
            PackCommands::Create {
                name,
                description,
                template,
                from,
                instance,
            } => pack::create(connection, tx, name, description, template, from, instance)?,
            PackCommands::Delete => {}
        },
    }

    Ok(())
}

fn match_event(rx: Receiver<Event>) -> Result<()> {
    let mut progresses = HashMap::new();
    let multi_progress = MultiProgress::new();

    for event in rx {
        match event {
            Event::Progress { id, title, total } => {
                let progress_bar = match total {
                    Some(total) => {
                        let progress_bar = ProgressBar::new(total);
                        progress_bar.set_style(ProgressStyle::with_template(&("{spinner} [".to_owned() + &title.bold().to_string() + "] {msg}\n {wide_bar} {percent:>3} % ({human_pos:>5} / {human_len:5})\nElapsed: {elapsed_precise}\tETA: {eta_precise}"))?);
                        progress_bar
                    }
                    None => {
                        let progress_bar = ProgressBar::new_spinner();
                        progress_bar.set_style(ProgressStyle::with_template(
                            &("{spinner} [".to_owned() + &title.bold().to_string() + "] {msg}"),
                        )?);
                        progress_bar
                    }
                };
                progress_bar.enable_steady_tick(Duration::from_millis(100));

                let progress_bar = multi_progress.add(progress_bar);
                progresses.insert(id, progress_bar);
            }
            Event::ProgressTick { id, message } => {
                progresses
                    .get(&id)
                    .unwrap()
                    .set_message(message.to_string());
                progresses.get(&id).unwrap().inc(1);
            }
            Event::ProgressFinish { id } => {
                progresses
                    .get(&id)
                    .unwrap()
                    .finish_with_message("✓ Finished".green().to_string());
            }
            Event::Confirm { tx, message } => {
                println!("[Confirm] Not yet implemented: {:?}, {:?}", tx, message)
            }
            Event::Select { tx, options } => {
                println!("[Select] Not yet implemented: {:?}, {:?}", tx, options)
            }
            Event::Warning { warning } => {
                multi_progress.println(
                    StatusOutput::new(Status::Warning, Message::new(&warning.to_string()))
                        .to_string(),
                )?;
            }
            Event::Log { message } => {
                multi_progress.println(message.to_string())?;
            }
        }
    }

    Ok(())
}

fn fallible(rx: Receiver<Event>, thread: JoinHandle<Result<()>>) -> Result<()> {
    match_event(rx)?;
    thread.join().unwrap()
}

fn main() {
    let (tx, rx) = mpsc::channel::<Event>();
    let cli = Cli::parse();

    let thread = thread::spawn(move || {
        let mut connection = db::init()?;
        let transaction = connection.transaction()?;
        let result = match_command(&cli.command, &transaction, &tx);
        transaction.commit()?;

        result
    });

    if let Err(error) = fallible(rx, thread) {
        StatusOutput::new(Status::Error, Message::new(&error.to_string())).print();
    }
}
