/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       msg.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   20.01.25, 12:53
 */

#[derive(Debug)]
pub struct Context {
    pub title: String,
    pub content: String,
}

impl Context {
    pub fn new(title: &str, content: &str) -> Self {
        Self {
            title: title.to_owned(),
            content: content.to_owned(),
        }
    }
}

#[derive(Debug)]
pub struct Message {
    pub msg: String,
    pub context: Vec<Context>,
}

impl Message {
    pub fn new(msg: &str) -> Self {
        Self {
            msg: msg.to_owned(),
            context: vec![],
        }
    }

    pub fn context(mut self, title: &str, content: &str) -> Self {
        self.context.push(Context::new(title, content));
        self
    }
}
