use errors::{Error, Result};
use failure::ResultExt;
use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;

// pub type Result<T> = ::std::result::Result<T, failure::Error>;

// #[derive(Debug, Fail)]
// pub enum Error {
//     #[fail(display = "Path does not exist: {}", name)]
//     DoesNotExist { name: String },

//     // TODO(): error should take a cause and a pathname.
//     #[fail(display = "{}", _0)]
//     IO(#[cause] io::Error),

//     #[fail(display = "Failed to derive basename for \"{}\"", name)]
//     Basename { name: String },
// }

// impl Error {
//     fn does_not_exist(path: PathBuf) -> Self {
//         // TODO: there has to be a better way to get a string here.
//         let name = path.to_str().unwrap();
//         Error::DoesNotExist {
//             name: String::from(name),
//         }
//     }

//     fn basename(suffix: &PathBuf) -> Self {
//         let name = suffix.to_str().unwrap();
//         Error::Basename {
//             name: String::from(name),
//         }
//     }
// }

// impl From<io::Error> for Error {
//     fn from(err: io::Error) -> Self {
//         Error::IO(err)
//     }
// }

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
pub fn mkdirp(path: &PathBuf) -> Result<()> {
    debug!("mkdirp - path exists: {}", path.exists());

    let res = match fs::create_dir_all(&path) {
        Ok(v) => Ok(v),
        Err(ref err) if exists(err) => Ok(()),
        Err(err) => Err(err),
    };

    res.map_err(Error::io).with_context(|_| Error::path(path))?;

    debug!("mkdirp - path exists: {}", path.exists());

    Ok(())
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
