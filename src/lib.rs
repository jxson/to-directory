// `error_chain!` recursion limit.
#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;
extern crate slog;

#[macro_use]
extern crate clap;

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;

pub mod cli;
pub mod errors;
