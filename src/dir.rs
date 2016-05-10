use error::{ToResult};
use std::env;
use std::path::{PathBuf};

pub fn resolve(pathname: &str) -> ToResult<PathBuf> {
    let mut absolute = try!(env::current_dir());
            absolute.push(pathname);

    let canonical = try!(absolute.canonicalize());

    return Ok(canonical);
}

#[cfg(test)]
mod tests {
    extern crate env_logger;

    use std::env;
    use super::*;

    #[test]
    fn relative() {
        let _ = env_logger::init();

        let actual = resolve("src").expect("should not fail");
        let mut expected = env::current_dir().expect("should not fail");
                expected.push("src");
        assert_eq!(actual, expected);

        let actual = resolve("src").expect("should not fail");
        assert_eq!(actual, expected);
    }

    #[test]
    fn dot() {
        let _ = env_logger::init();

        let actual = resolve(".").expect("should not fail");
        let expected = env::current_dir().expect("should not fail");

        assert_eq!(actual, expected);
    }

    #[test]
    fn dot_dot() {
        let _ = env_logger::init();

        let actual = resolve("../to-directory/src").expect("should not fail");
        let mut expected = env::current_dir().expect("should not fail");
                expected.pop();
                expected.push("to-directory");
                expected.push("src");

        assert_eq!(actual, expected);
    }
}
