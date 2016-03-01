#[macro_use]
extern crate clap;

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
    let operation = parse(program);

    println!("operation.name: {}", operation.name);
}

struct Operation {
    name: String,
    directory: PathBuf,
}

impl Operation {
    fn new(name: &str, directory: &PathBuf) -> Operation {
        Operation{ name: "foo".to_string(), directory: PathBuf::from(".")}
    }
}
// pub fn new(home: &path::PathBuf) -> ToResult<Store> {
//     let mut directory = path::PathBuf::from(home);
//             directory.push(".to");
//
//     match bootstrap(&directory) {
//         Ok(db) => return Ok(Store { db: Some(db) }),
//         Err(err) => return Err(err),
//     };
// }

fn parse(cli: clap::App) -> Operation {
    let matches = cli.get_matches();
    let pathame = matches.value_of("directory").unwrap_or("");
    let directory = resolve(pathame);
    let basename = directory.file_stem().unwrap().to_str().unwrap();
    let name = matches.value_of("name").unwrap_or(basename);

    println!("directory: {:?}", directory);
    println!("name: {:?}", name);

    return Operation{ name: "foo".to_string(), directory: PathBuf::from(".")}
}

fn resolve(pathname: &str) -> PathBuf {
    // TODO: Use a custom results tuple instead of panic!.
    let mut absolute = match env::current_dir() {
        Ok(value) => value,
        Err(err) => panic!(err),
    };

    // Don't default to "." since it will be a literal translation creating
    // dumb directoris like "/foo/bar/."
    if pathname != "." {
        absolute.push(pathname);
    }

    return absolute;
}
