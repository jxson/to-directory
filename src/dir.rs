use failure;
use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;

pub type Result<T> = ::std::result::Result<T, failure::Error>;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Path does not exist: {}", name)]
    DoesNotExist { name: String },

    // TODO(): error should take a cause and a pathname.
    #[fail(display = "{}", _0)]
    IO(#[cause] io::Error),

    #[fail(display = "Failed to derive basename for \"{}\"", name)]
    Basename { name: String },
}

impl Error {
    fn does_not_exist(path: PathBuf) -> Self {
        // TODO: there has to be a better way to get a string here.
        let name = path.to_str().unwrap();
        Error::DoesNotExist {
            name: String::from(name),
        }
    }

    fn basename(suffix: &PathBuf) -> Self {
        let name = suffix.to_str().unwrap();
        Error::Basename {
            name: String::from(name),
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IO(err)
    }
}

pub fn resolve(path: PathBuf) -> Result<PathBuf> {
    let absolute = match env::current_dir() {
        Ok(mut value) => {
            value.push(path.to_path_buf());
            value
        }
        Err(err) => bail!(Error::from(err)),
    };

    if !absolute.exists() {
        bail!(Error::does_not_exist(absolute));
    }

    match absolute.canonicalize() {
        Ok(value) => Ok(value),
        Err(err) => bail!(Error::from(err)),
    }
}

pub fn basename(path: &PathBuf) -> Result<String> {
    match path.file_stem() {
        None => bail!(Error::basename(path)),
        Some(stem) => {
            let os_string = stem.to_os_string();

            match os_string.into_string() {
                Ok(string) => Ok(string),
                // TODO(): add cause to error.
                Err(_) => bail!(Error::basename(path)),
            }
        }
    }
}

/// A function that acts like `mkdir -p`.
pub fn mkdirp(directory: &PathBuf) -> Result<()> {
    match fs::create_dir_all(&directory) {
        Ok(_) => Ok(()),
        Err(ref err) if exists(err) => Ok(()),
        Err(err) => bail!(Error::from(err)),
    }
}

fn exists(err: &io::Error) -> bool {
    err.kind() == io::ErrorKind::AlreadyExists
}

#[cfg(test)]
mod test {
    extern crate tempdir;

    use self::tempdir::TempDir;
    use super::*;

    #[test]
    fn mkdirp_existing() {
        let path = TempDir::new("mkdirp-test")
            .map(|dir| dir.into_path())
            .unwrap();
        assert!(path.exists());
        assert!(mkdirp(&path).is_ok());
        assert!(path.exists());
    }

    #[test]
    fn mkdirp_non_existing() {
        let path = TempDir::new("existing-dir")
            .map(|dir| dir.into_path().join("non-existing"))
            .unwrap();
        assert_eq!(path.exists(), false);
        assert!(mkdirp(&path).is_ok());
        assert!(path.exists());
    }

    // #[test]
    // fn mkdirp_error() {
    //     let path = PathBuf::from("/should-not-have-premissions");
    //     let err = mkdirp(&path).err().unwrap();
    //     assert_eq!(
    //         format!("{}", ErrorKind::CreateDirError(path)),
    //         // format!("{}", Error::from(err)),
    //         format!("{}", err)
    //     );
    // }
}
