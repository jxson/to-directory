use crate::errors::{Error, Result};
use failure::ResultExt;
use mkdirp::mkdirp as _mkdirp;
use std::env;
use std::path::{Path, PathBuf};

pub fn resolve(path: PathBuf) -> Result<PathBuf> {
    let path = env::current_dir()
        .map(|mut p| {
            p.push(path.to_path_buf());
            p
        })
        .map_err(Error::io)?;

    let path = path
        .canonicalize()
        .map_err(Error::io)
        .with_context(|_| Error::path(path))?;

    Ok(path)
}

pub fn basename(path: &PathBuf) -> Result<String> {
    let os_string = path
        .file_stem()
        .map(|stem| stem.to_os_string())
        .ok_or_else(|| format_err!("failed to get file stem"))
        .with_context(|_| Error::path(path))?;

    let string = os_string
        .into_string()
        .map_err(|_| format_err!("failed to convert os string"))
        .with_context(|_| Error::path(path))?;

    Ok(string)
}

/// A function that acts like `mkdir -p`.
pub fn mkdirp<P: AsRef<Path>>(path: &P) -> Result<()> {
    _mkdirp(path)
        .map_err(Error::io)
        .with_context(|_| Error::path(path))?;

    Ok(())
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
}
