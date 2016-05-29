use error::{ToResult};
use std::env;
use std::path::{PathBuf};

pub fn resolve(pathname: &str) -> ToResult<PathBuf> {
    let mut absolute = try!(env::current_dir());
            absolute.push(pathname);

    let canonical = try!(absolute.canonicalize());

    return Ok(canonical);
}

pub fn basename(path: &PathBuf) -> ToResult<String> {
    if let Some(stem) = path.file_stem() {
        let os_string = stem.to_os_string();

        return match os_string.into_string() {
            Ok(string) => Ok(string),
            Err(_) => Ok(String::from("")),
        };
    }

    return Ok(String::from(""));
}

pub fn config() -> ToResult<PathBuf> {
    let mut directory = match env::home_dir() {
        Some(value) => value,
        None => panic!("TODO: Custom error - unable to locate home directory."),
    };

    directory.push(".to");

    return Ok(directory);
}


#[cfg(test)]
mod tests {
    extern crate env_logger;

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
