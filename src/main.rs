#[macro_use]
extern crate clap;
extern crate to;

use to::{ToResult, ToError};
use clap::{App, Arg, ArgGroup};
use std::path::PathBuf;
use std::env;
use std::fs;

fn main() {
    let version = crate_version!();
    let actions = [
        Arg::with_name("delete")
            .help("Delete bookmark")
            .long("delete")
            .short("d")
    ];
    let app = App::new("to")
                    .about("Bookmark directories")
                    .version(version)
                    .author("Jason Campbell <jason@artifact.sh>")
                    .arg(Arg::with_name("name")
                        .help("Name of the bookamrk")
                        .index(1))
                    .arg(Arg::with_name("directory")
                        .help("Path of the bookamrk")
                        .index(2))
                    .args(&actions);

    let cli = to::cli::CLI::new(actions, app);

    let request = match cli.run() {
        Ok(value) => value,
        Err(err) => panic!(err),
    };

    println!("CLI Request: {:?}", request);
}
