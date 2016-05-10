use error::{ToResult};
use std::env;
use std::fs;
use std::path::{PathBuf};

pub fn resolve(pathname: &str) -> ToResult<PathBuf> {
    let mut absolute = try!(env::current_dir());

    return Ok(PathBuf::from("bar"));
}
