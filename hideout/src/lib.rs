#![feature(or_patterns, proc_macro_hygiene, ptr_offset_from)]

#[macro_use]
extern crate failure;
pub mod http;
mod logger;
mod util;

pub use logger::Logger;
pub use util::{AssetStore, Config};
