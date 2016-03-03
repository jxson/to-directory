#[macro_use]
extern crate clap;
extern crate to;

use to::{ToResult, ToError};
use to::cli::{Request};
use clap::{App, Arg};
use std::path::PathBuf;
use std::env;
use std::fs;

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
    let request = match parse(program) {
        Ok(value) => value,
        Err(err) => panic!(err),
    };

    println!("CLI Request: {:?}", request);
}

fn parse(cli: clap::App) -> ToResult<Request> {
    let matches = cli.get_matches();
    let pathname = matches.value_of("directory").unwrap_or("");

    let directory = match resolve(pathname) {
        Ok(value) => value,
        Err(err) => return Err(err),
    };

    // Validate that the directory exists and is a directory.
    // TODO: Move this into the resolve step.
    if !is_valid_dir(&directory) {
        panic!("invalid directory");
    }

    let name = match matches.value_of("name") {
        Some(value) => value,
        // directory.file_stem().map(|stem| stem.to_str()).unwrap()
        None => "",
    };

    println!("directory: {:?}", directory);
    println!("name: {:?}", name);

    let req = Request{ name: name.to_string(), directory: directory };

    return Ok(req);
}

fn resolve(pathname: &str) -> ToResult<PathBuf> {
    // TODO: Use a custom results tuple instead of panic!.
    let mut absolute = match env::current_dir() {
        Ok(value) => value,
        Err(err) => return Err(ToError::Io(err)),
    };

    // Don't default to "." since it will be a literal translation creating
    // dumb directories like "/foo/bar/."
    if pathname != "." {
        absolute.push(pathname);
    }

    return Ok(absolute);
}

fn is_valid_dir(directory: &PathBuf) -> bool {
    let path = directory.clone();

    let metadata = match fs::metadata(path) {
        Ok(value) => value,
        Err(_) => return false,
    };

    if metadata.is_dir() {
        return true;
    }

    return false;
}
