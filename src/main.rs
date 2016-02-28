#[macro_use]
extern crate clap;

use clap::{App, Arg};
use std::path::PathBuf;

fn main() {
    // Use the clap crate to handle argument parsing.
    let version = crate_version!();
    let program = App::new("to")
                    .about("Bookmark directories")
                    .version(version)
                    .author("Jason Campbell <jason@artifact.sh>")
                    .arg(Arg::with_name("name")
                        .help("Name of the bookamrk")
                        .index(1))
                    .arg(Arg::with_name("directory")
                        .help("Path of the bookamrk")
                        .index(2));

    // Take the matches from clap and convert them into name, directory, and
    // action.
    let operation = parse(program);
}

struct Operation {
    name: String,
    directory: PathBuf,
}

fn parse(cli: clap::App) -> Operation {
    let matches = cli.get_matches();

    if let Some(name) = matches.value_of("name") {
        println!("A name was passed in: {}", name);
    }

    if let Some(directory) = matches.value_of("directory") {
        println!("A name was passed in: {}", directory);
    }

    return Operation{ name: "foo".to_string(), directory: PathBuf::from(".")}
}
