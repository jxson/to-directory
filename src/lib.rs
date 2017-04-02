// `error_chain!` recursion limit.
#![recursion_limit = "1024"]

#[macro_use]
extern crate clap;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate slog;
extern crate chrono;
extern crate slog_json;
extern crate slog_stream;

pub mod cli;
pub mod dir;
pub mod errors;
pub mod logger;
