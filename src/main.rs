#[macro_use]
extern crate clap;

mod cli;

fn main() {
    let request = match cli::get_request() {
        Ok(value) => value,
        Err(err) => panic!(err),
    };

    println!("CLI Request: {:?}", request);
}
