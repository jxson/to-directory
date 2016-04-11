extern crate clap;

use types::{ToResult, ToError};
use std::path::PathBuf;
use std::fmt;
use std::env;
use std::fs;
use std::option::Option;

#[derive(Debug)]
pub enum Action {
    Get,
    Put,
    Delete,
    List,
    Last,
    ChangeDirectory,
}

pub fn parse_matches(matches: clap::ArgMatches) -> ToResult<Request> {
    let (get, put, delete, list, last) = (matches.is_present("get"),
                                          matches.is_present("put"),
                                          matches.is_present("delete"),
                                          matches.is_present("list"),
                                          matches.is_present("last"));

    let action = match (get, put, delete, list, last) {
        (true, _, _, _, _) => Action::Get,
        (_, true, _, _, _) => Action::Put,
        (_, _, true, _, _) => Action::Delete,
        (_, _, _, true, _) => Action::List,
        (_, _, _, _, true) => Action::Last,
        _ => Action::ChangeDirectory,
    };

    // to --save,--put # Save current dir as a bookmark
    // to --save,--put foo # Save current directory as bookmark foo
    // to --delete foo # Delete bookmark foo
    // to --info foo # Show details of the bookmark foo
    // to --list # List all the bookmarks
    // to foo # Go to the foo bookmark
    // reserve "-" so it can be used later.
    // to - # go to the last bookmark you visited

    println!("action: {:?}", action);

    let directory = match resolve(matches.value_of("directory")) {
        Ok(value) => value,
        Err(err) => return Err(err),
    };

    println!("directory: {:?}", directory);

    let dir = directory.clone();
    let basedir = match dir.file_stem() {
        Some(value) => value.to_str(),
        None => panic!("TODO: I dont even"),
    };

    let name = matches.value_of("name")
                      .unwrap_or(basedir.unwrap())
                      .to_string();

    println!("basedir {:?}", basedir);
    println!("name {:?}", name);

    let req = Request::new(name, directory, action);
    return Ok(req);
}

fn resolve(pathname: Option<&str>) -> ToResult<PathBuf> {
    let pathname = match pathname {
        Some(value) => value,
        None => "",
    };

    let mut absolute = match env::current_dir() {
        Ok(value) => value,
        Err(err) => return Err(ToError::Io(err)),
    };

    // Don't default to "." since it will be a literal translation creating
    // dumb directories like "/foo/bar/."
    if pathname != "." {
        absolute.push(pathname);
    }

    // TODO: figure out a better way to deal with this moved value.
    let resolved = absolute.clone();

    match exists(absolute) {
        Ok(value) => {
            match value {
                true => return Ok(resolved),
                false => panic!("TODO: Does not exist error!"),
            }
        }
        Err(err) => Err(err),
    }
}

fn exists(directory: PathBuf) -> ToResult<bool> {
    let metadata = match fs::metadata(directory) {
        Ok(value) => value,
        Err(err) => return Err(ToError::Io(err)),
    };

    if metadata.is_dir() {
        return Ok(true);
    }

    return Ok(false);
}

#[derive(Debug)]
pub struct Request {
    pub name: String,
    pub directory: PathBuf,
    pub action: Action,
}

impl Request {
    fn new(name: String, directory: PathBuf, action: Action) -> Request {
        Request {
            name: name,
            directory: directory,
            action: action,
        }
    }
}
