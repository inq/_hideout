#![feature(or_patterns, proc_macro_hygiene, ptr_offset_from)]

#[macro_use]
extern crate failure;
pub mod http;
pub mod model;
pub mod util;
