#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;
extern crate env_logger;

mod cli;
mod dir;
mod error;

fn main() {
    // TODO: Add a match here and have a nice error message.
    env_logger::init().unwrap();

    let request = match cli::Request::get() {
        Ok(value) => value,
        Err(err) => panic!(err),
    };

    println!("CLI Request: {:?}", request);
}
