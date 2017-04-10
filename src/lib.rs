// `error_chain!` recursion limit.
#![recursion_limit = "1024"]

#![feature(plugin)]
#![plugin(indoc)]

#[macro_use]
extern crate clap;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate slog;
extern crate slog_json;
extern crate chrono;
extern crate bincode;
extern crate time;

pub mod cli;
pub mod dir;
pub mod errors;
pub mod logger;
pub mod database;

// Get the current time in milliseconds.
pub fn now() -> u64 {
    let timespec = time::now_utc().to_timespec();
    let seconds = timespec.sec as u64 * 1000;
    let milliseconds_offset = timespec.nsec as u64 / 1000 / 1000;

    seconds + milliseconds_offset
}
