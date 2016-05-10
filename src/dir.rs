use std::path::{PathBuf};
use error::{ToResult};

pub fn resolve(pathname: &str) -> ToResult<PathBuf> {
    return Ok(PathBuf::from("bar"));
}
