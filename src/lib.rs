// `error_chain!` recursion limit.
#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;

pub mod cli;
pub mod errors;
