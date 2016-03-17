use std::path::PathBuf;
use std::env;
use types::{ToResult, ToError};

pub fn config() -> ToResult<PathBuf> {
    let mut directory = match env::home_dir() {
        Some(value) => value,
        None => panic!("TODO: Custom error - unable to locate home directory."),
    };

    directory.push(".to");

    return Ok(directory);
}
