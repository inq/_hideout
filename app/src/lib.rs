#![feature(proc_macro_hygiene, try_blocks)]

pub mod controllers;
pub mod handlers;
pub mod models;

pub type Context = hideout::context::Context<models::Session>;
pub type ServerState = hideout::context::ServerState<models::Session>;
