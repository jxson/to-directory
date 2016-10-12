use error::{ToResult, ToError};
use std::env;
use std::path::{PathBuf};
use std::fs;
use std::io;

pub fn resolve(pathname: &str) -> ToResult<PathBuf> {
    let mut absolute = try!(env::current_dir());
            absolute.push(pathname);

    let canonical = try!(absolute.canonicalize());

    return Ok(canonical);
}

pub fn basename(path: &PathBuf) -> Option<String> {
    if let Some(stem) = path.file_stem() {
        let os_string = stem.to_os_string();

        return match os_string.into_string() {
            Ok(string) => Some(string),
            Err(_) => None,
        }
    }

    Some(String::from(""))
}

pub fn config() -> ToResult<PathBuf> {
    let mut directory = match env::home_dir() {
        Some(value) => value,
        None => return Err(ToError::UnknownHomeDirectory),
    };

    directory.push(".to");

    if let Err(err) = mkdirp(&directory) {
        return Err(err);
    }

    return Ok(directory);
}

fn mkdirp(directory: &PathBuf) -> ToResult<()> {
    match fs::create_dir(&directory) {
        Ok(_) => return Ok(()),
        Err(ref err) if exists(err) => return Ok(()),
        Err(err) => Err(ToError::Io(err)),
    }
}

fn exists(err: &io::Error) -> bool {
    return err.kind() == io::ErrorKind::AlreadyExists;
}

pub fn db() -> ToResult<PathBuf> {
    let mut pathname = try!(config());
            pathname.push("db");
    return Ok(pathname);
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::path::{PathBuf};
    use super::*;

    #[test]
    fn resolve_relative() {
        let actual = resolve("src").expect("should not fail");
        let mut expected = env::current_dir().expect("should not fail");
                expected.push("src");
        assert_eq!(actual, expected);

        let actual = resolve("src").expect("should not fail");
        assert_eq!(actual, expected);
    }

    #[test]
    fn resolve_dot() {
        let actual = resolve(".").expect("should not fail");
        let expected = env::current_dir().expect("should not fail");

        assert_eq!(actual, expected);
    }

    #[test]
    fn resolve_dot_dot() {
        let actual = resolve("../to-directory/src").expect("should not fail");
        let mut expected = env::current_dir().expect("should not fail");
                expected.pop();
                expected.push("to-directory");
                expected.push("src");

        assert_eq!(actual, expected);
    }

    #[test]
    fn basename_absolute() {
        let pathname = PathBuf::from("/foo/bar");
        let basename = basename(&pathname).expect("should not fail");

        assert_eq!(basename, String::from("bar"));
    }
}
