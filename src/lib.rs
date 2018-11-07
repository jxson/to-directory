#![feature(extern_prelude)]

extern crate bincode;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate serde_derive;
extern crate time;
#[macro_use]
extern crate log;
extern crate loggerv;
#[macro_use]
extern crate failure;

pub mod cli;
pub mod database;
pub mod dir;
pub mod errors;

/// Get the current time in milliseconds.
pub fn now() -> u64 {
  let timespec = time::now_utc().to_timespec();
  let seconds = timespec.sec as u64 * 1000;
  let milliseconds_offset = timespec.nsec as u64 / 1000 / 1000;

  seconds + milliseconds_offset
}
