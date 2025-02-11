/*
 * Copyright (c) 2024-2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       main.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   11.02.25, 17:37
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
use minepatch::hash::{HashMessage, HashProcess};
use minepatch::instance::{InstanceMessage, InstanceProcess};
use minepatch::msg::Process;
use minepatch::pack::{PackMessage, PackProcess};
use minepatch::patch::{PatchMessage, PatchProcess};
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
            InstanceCommands::Apply { name, patch } => {
                instance::apply(connection, tx, name, patch)?
            }
            InstanceCommands::List { name } => instance::list(connection, tx, name)?,
            InstanceCommands::Link { path, name, pack } => {
                instance::link(connection, tx, path, name, pack)?;
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
            } => patch::create(connection, tx, name, pack, dependency)?,
            PatchCommands::Exclude {
                name,
                pack,
                mod_hash,
            } => patch::exclude(connection, tx, name, pack, mod_hash)?,
            PatchCommands::Generate { name, instance } => {
                patch::generate(connection, tx, name, instance)?
            }
            PatchCommands::Include {
                name,
                pack,
                mod_hash,
            } => patch::include(connection, tx, name, pack, mod_hash)?,
            PatchCommands::List { name, pack } => patch::list(connection, name, pack)?,
            PatchCommands::Simulate {
                name,
                pack,
                dir_hash,
            } => patch::simulate(connection, tx, name, pack, dir_hash)?,
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

fn match_process(process: &Process) -> &'static str {
    match process {
        Process::Hash(process) => match process {
            HashProcess::HashFiles => "Hashing mod files",
        },
        Process::Instance(process) => match process {
            InstanceProcess::Detect => "Detecting patch and pack",
        },
        Process::Pack(process) => match process {
            PackProcess::Create => "Creating pack",
            PackProcess::AddModFiles => "Adding mod files",
        },
        Process::Patch(process) => match process {
            PatchProcess::Simulate => "Simulating patch",
        },
        Process::Mod(process) => "",
    }
}

fn match_message(message: &Message) -> &'static str {
    match message {
        Message::Hash(message) => match message {
            HashMessage::HashFilesStatus(context) => "Not yet implemented",
        },
        Message::Instance(message) => match message {
            InstanceMessage::LinkSuccess(context) => "Not yet implemented",
        },
        Message::Pack(message) => match message {
            PackMessage::AddModFileStatus(context) => "Not yet implemented",
        },
        Message::Patch(message) => match message {
            PatchMessage::SimulateStatus(context) => "Not yet implemented",
        },
        Message::Mod(message) => "",
    }
}

// fn match_context(context: &Context) -> String {
//     match context {
//         Context::Hash(context) => match context {
//             HashContext::Path(path) => path.display().to_string(),
//         },
//         Context::Instance(context) => match context {
//             InstanceContext::SuccessObj(instance) => format!("'{}'", instance.name),
//         },
//         Context::Pack(context) => match context {
//             PackContext::Path(path) => path.display().to_string(),
//             PackContext::Hash(hash) => hash.to_owned(),
//         },
//         Context::Patch(context) => match context {
//             PatchContext::Name(name) => name.to_owned(),
//         },
//         Context::Mod(context) => "".to_owned(),
//     }
// }

fn match_event(rx: Receiver<Event>) -> Result<()> {
    let mut progresses = HashMap::new();
    let multi_progress = MultiProgress::new();

    for event in rx {
        match event {
            Event::Progress { process, total } => {
                let progress_bar = progresses.remove(&process);
                if progress_bar.is_some() {
                    let progress_bar: ProgressBar = progress_bar.unwrap();
                    progress_bar.finish_and_clear();
                }

                let progress_bar = match total {
                    Some(total) => {
                        let progress_bar = ProgressBar::new(total);
                        progress_bar.set_style(
                            ProgressStyle::with_template(&format!(
                                "{{spinner}} [{}] {{msg}}\n{{wide_bar}} {{percent:>3}} % ({{human_pos:>5}} / {{human_len:5}})\nElapsed: {{elapsed_precise}}\tETA: {{eta_precise}}",
                                match_process(&process).bold()
                            ))?.tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏"),
                        );
                        progress_bar
                    }
                    None => {
                        let progress_bar = ProgressBar::new_spinner();
                        progress_bar.set_style(
                            ProgressStyle::with_template(&format!(
                                "{{spinner}} [{}] {{msg}}",
                                match_process(&process).bold()
                            ))?
                            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏"),
                        );
                        progress_bar
                    }
                };
                progress_bar.enable_steady_tick(Duration::from_millis(100));

                let progress_bar = multi_progress.add(progress_bar);
                progresses.insert(process, progress_bar);
            }
            Event::ProgressTick { process, message } => {
                let progress_bar = progresses.get(&process).unwrap();

                progress_bar.set_message(match_message(&message));
                progress_bar.inc(1);
            }
            Event::ProgressFinish { process } => {
                let progress_bar = progresses.get(&process).unwrap();
                progress_bar.set_style(ProgressStyle::with_template(&format!(
                    "{} [{}]",
                    "✓".green(),
                    match_process(&process).bold()
                ))?);
                progress_bar.finish_with_message("Finished");
            }
            Event::Confirm { tx, message } => {
                println!("[Confirm] Not yet implemented: {:?}, {:?}", tx, message)
            }
            Event::Select { tx, options } => {
                println!("[Select] Not yet implemented: {:?}, {:?}", tx, options)
            }
            Event::Success { message } => multi_progress.println(
                StatusOutput::new(Status::Success, match_message(&message).to_owned()).to_string(),
            )?,
            Event::Warning { warning } => {
                multi_progress
                    .println(StatusOutput::new(Status::Warning, warning.to_string()).to_string())?;
            }
            Event::Log { message } => {
                multi_progress.println(match_message(&message))?;
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
        if result.is_ok() {
            transaction.commit()?
        };

        result
    });

    if let Err(error) = fallible(rx, thread) {
        StatusOutput::new(Status::Error, error.to_string()).print();
    }
}
