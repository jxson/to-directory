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
        Arg::with_name("get")
            .help("Show bookmark information")
            .long("info")
            .short("i"),

        // to --save,--put # Save current dir as a bookmark
        // to --save,--put foo # Save current directory as bookmark foo
        Arg::with_name("put")
            .help("Save bookmark")
            .long("save")
            .short("s"),

        // to --delete foo # Delete bookmark foo
        Arg::with_name("delete")
            .help("Delete bookmark")
            .long("delete")
            .short("d"),

        Arg::with_name("list")
            .help("List all bookmarks")
            .long("list")
            .short("l"),

        Arg::with_name("last")
            .help("Change current working directory to last")
            .long("last"),
    ];
    let matches = App::new("to")
                    .about("Bookmark directories")
                    .version(version)
                    .author("Jason Campbell <jason@artifact.sh>")
                    .arg(Arg::with_name("name")
                        .help("Name of the bookamrk")
                        .index(1))
                    .arg(Arg::with_name("directory")
                        .help("Path of the bookamrk")
                        .index(2))
                    .args(&actions)
                    .get_matches();

    let request = match to::cli::parse_matches(matches) {
        Ok(value) => value,
        Err(err) => panic!(err),
    };

    println!("CLI Request: {:?}", request);
}
