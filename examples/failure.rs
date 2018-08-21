#[macro_use]
extern crate failure;
extern crate dirs;

use failure::{Context, Fail, ResultExt};
use std::env;
use std::fmt;
use std::fmt::Display;
use std::io;
use std::path::PathBuf;

fn main() {
    let does_not_exist = PathBuf::from("does-not-exist");

    // for cause in Fail::iter_causes(&resolve(does_not_exist).unwrap_err()) {
    //     println!("{}", cause);
    // }

    let err = resolve(does_not_exist).unwrap_err();
    println!("{:?}, {:?}", err.cause(), err.backtrace())
}

#[derive(Debug, Fail)]
pub struct Error {
    ctx: Context<ErrorKind>,
}

#[derive(Debug, Fail)]
pub enum ErrorKind {
    #[fail(display = "{:?}", _0)]
    Path(PathBuf),
    #[fail(display = "{}", _0)]
    IO(#[fail(cause)] io::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.ctx, f)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error {
            ctx: Context::new(ErrorKind::IO(err)),
        }
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        Error {
            ctx: Context::new(kind),
        }
    }
}

impl From<Context<ErrorKind>> for Error {
    fn from(ctx: Context<ErrorKind>) -> Error {
        Error { ctx: ctx }
    }
}

pub fn resolve(path: PathBuf) -> Result<PathBuf, failure::Error> {
    env::current_dir()
        .map(|mut full_path| {
            full_path.push(path.to_path_buf());
            full_path
        }).and_then(|p| {
            if p.exists() {
                Err(io::Error::from(io::ErrorKind::NotFound))
            } else {
                Ok(p)
            }
        }).and_then(|p| p.canonicalize())
    // .map_err(Error::from)
    // .context(ErrorKind::Path(path))

    // if !absolute.exists() {
    //     let err = Error::from(ErrorKind::Path(path));
    //     return Err(err);
    // }

    // Ok(absolute)

    // match absolute.canonicalize() {
    //     Ok(value) => Ok(value),
    //     Err(err) => bail!(Error::from(err)),
    // }
}
