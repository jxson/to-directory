#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;
extern crate env_logger;

mod cli;
mod dir;
mod database;
mod error;
#[macro_use]
mod logger;

use database::{Database};

fn main() {
    let request = match cli::Request::get() {
        Ok(value) => value,
        Err(err) => panic!(err),
    };

    logger::init(request.verbose);
    log!("Logger works!");
    log!("request: {:?}", request);
}
