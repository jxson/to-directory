use std::env;
use std::path::PathBuf;
// use std::fs;
// use std::io;

use errors::*;

pub fn resolve(path: PathBuf) -> Result<PathBuf> {
    let absolute = match env::current_dir() {
        Ok(mut value) => {
            value.push(path.to_path_buf());
            value
        }
        Err(_) => bail!(ErrorKind::ResolveError(path)),
    };

    match absolute.canonicalize() {
        Ok(value) => Ok(value),
        Err(_) => bail!(ErrorKind::ResolveError(path)),
    }
}

pub fn basename(path: &PathBuf) -> Result<String> {
    match path.file_stem() {
        None => bail!(ErrorKind::FailedToDeriveBasename(path.to_path_buf())),
        Some(stem) => {
            let os_string = stem.to_os_string();

            match os_string.into_string() {
                Ok(string) => Ok(string),
                Err(_) => bail!(ErrorKind::FailedToDeriveBasename(path.to_path_buf())),
            }
        }
    }
}

/// Get the default config directory.
///
/// ```
/// use std::env;
/// use to::dir;
///
/// let mut directory = env::home_dir().unwrap();
///         directory.push(".to");
///
/// assert_eq!(dir::config(), Some(directory));
/// ```
pub fn config() -> Option<PathBuf> {
    env::home_dir().map(|mut home| {
                            home.push(".to");
                            home
                        })
}

// fn mkdirp(directory: &PathBuf) -> Result<()> {
//     match fs::create_dir(&directory) {
//         Ok(_) => return Ok(()),
//         Err(ref err) if exists(err) => return Ok(()),
//         Err(err) => bail!(err),
//     }
// }

// fn home() -> Result<PathBuf> {
//     match env::home_dir() {
//         Some(value) => Ok(value),
//         None => bail!(ErrorKind::UnknownHomeDirectory),
//     }
// }
//
//
// fn exists(err: &io::Error) -> bool {
//     return err.kind() == io::ErrorKind::AlreadyExists;
// }
