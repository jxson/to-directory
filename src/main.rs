#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;
extern crate env_logger;

mod cli;
mod dir;
mod error;
#[macro_use]
mod logger;

fn main() {
    let request = match cli::Request::get() {
        Ok(value) => value,
        Err(err) => panic!(err),
    };

    logger::init(request.verbose);
    debug!("Logger works!");
}
