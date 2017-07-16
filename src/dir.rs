use std::env;
use std::path::PathBuf;
use std::fs;
use std::io;

use errors::*;

pub fn resolve(path: PathBuf) -> Result<PathBuf> {
    let absolute = match env::current_dir() {
        Ok(mut value) => {
            value.push(path.to_path_buf());
            value
        }
        Err(_) => bail!(ErrorKind::CurrentDirectoryError(path)),
    };

    if !absolute.exists() {
        bail!(ErrorKind::PathDoesNotExistError(absolute));
    }

    match absolute.canonicalize() {
        Ok(value) => Ok(value),
        Err(_) => bail!(ErrorKind::ResolveError(path)),
    }
}

pub fn basename(path: &PathBuf) -> Result<String> {
    match path.file_stem() {
        None => bail!(ErrorKind::BasenameError(path.to_path_buf())),
        Some(stem) => {
            let os_string = stem.to_os_string();

            match os_string.into_string() {
                Ok(string) => Ok(string),
                Err(_) => bail!(ErrorKind::BasenameError(path.to_path_buf())),
            }
        }
    }
}

/// A function that acts like `mkdir -p`.
pub fn mkdirp(directory: &PathBuf) -> Result<()> {
    match fs::create_dir_all(&directory) {
        Ok(_) => Ok(()),
        Err(ref err) if exists(err) => Ok(()),
        Err(_) => bail!(ErrorKind::CreateDirError(directory.to_path_buf())),
    }
}

fn exists(err: &io::Error) -> bool {
    err.kind() == io::ErrorKind::AlreadyExists
}

#[cfg(test)]
mod test {
    extern crate tempdir;

    use super::*;
    use self::tempdir::TempDir;

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

    #[test]
    fn mkdirp_error() {
        let path = PathBuf::from("/should-not-have-premissions");
        let err = mkdirp(&path).err().unwrap();
        assert_eq!(format!("{}", ErrorKind::CreateDirError(path)),
                   format!("{}", err));
    }
}
