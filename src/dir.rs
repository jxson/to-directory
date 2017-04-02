use std::env;
use std::path::{PathBuf};
use std::fs;
use std::io;

use errors::*;

pub fn resolve(pathname: String) -> Result<PathBuf> {
    let mut absolute = try!(env::current_dir());
            absolute.push(pathname);

    let canonical = try!(absolute.canonicalize());

    return Ok(canonical);
}

pub fn basename(path: &PathBuf) -> Result<String> {
    match path.file_stem() {
        None => bail!(ErrorKind::FailedToDeriveName),
        Some(stem) => {
            let os_string = stem.to_os_string();

            match os_string.into_string() {
                Ok(string) => Ok(string),
                Err(_) => bail!(ErrorKind::FailedToDeriveName),
            }
        }
    }
}

pub fn config(directory: Option<String>) -> Result<PathBuf> {
    let path = match directory {
        Some(_) => bail!("--config not yet supported."),
        None => {
            let mut path = try!(home());
            path.push(".to");
            path
        },
    };

    if let Err(err) = mkdirp(&path) {
        return Err(err);
    }

    return Ok(path);
}

fn home() -> Result<PathBuf> {
    match env::home_dir() {
        Some(value) => return Ok(value),
        None => bail!(ErrorKind::UnknownHomeDirectory),
    }
}

fn mkdirp(directory: &PathBuf) -> Result<()> {
    match fs::create_dir(&directory) {
        Ok(_) => return Ok(()),
        Err(ref err) if exists(err) => return Ok(()),
        Err(err) => bail!(err),
    }
}

fn exists(err: &io::Error) -> bool {
    return err.kind() == io::ErrorKind::AlreadyExists;
}
