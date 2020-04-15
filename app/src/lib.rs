#![feature(proc_macro_hygiene)]

pub mod controllers;
pub mod handlers;
pub mod models;

pub type Context = hideout::context::Context<models::Session>;
