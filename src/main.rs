#[macro_use]
extern crate clap;
extern crate to;

use to::{ToResult, ToError};
use to::cli::{Request};
use clap::{App, Arg};
use std::path::PathBuf;
use std::env;

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

    let name = match matches.value_of("name") {
        Some(value) => value,
        // directory.file_stem().map(|stem| stem.to_str()).unwrap()
        None => "",
    };

    // let pathame = .unwrap_or("");
    // let directory = resolve(pathame);
    // let basename = directory.file_stem().unwrap().to_str().unwrap();
    // let name = matches.value_of("name").unwrap_or(basename);

    println!("directory: {:?}", directory);
    println!("name: {:?}", name);

    let req = Request{ name: "foo".to_string(), directory: PathBuf::from(".")};

    return Ok(req);
}

fn resolve(pathname: &str) -> ToResult<PathBuf> {
    // TODO: Use a custom results tuple instead of panic!.
    let mut absolute = match env::current_dir() {
        Ok(value) => value,
        Err(err) => return Err(err),
    };

    // Don't default to "." since it will be a literal translation creating
    // dumb directories like "/foo/bar/."
    if pathname != "." {
        absolute.push(pathname);
    }

    return Ok(absolute);
}
