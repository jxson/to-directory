#[macro_use]
extern crate failure;
extern crate to;

use failure::{Backtrace, Context, Fail, ResultExt};
use std::env;
use std::fmt;
use std::fmt::Display;
use std::io;
use std::path::{Path, PathBuf};

fn main() {
    let path = PathBuf::from("does-not-exist");

    // for cause in Fail::iter_causes(&resolve(does_not_exist).unwrap_err()) {
    //     println!("{}", cause);
    // }

    match resolve(path) {
        Ok(path) => println!("success: {:?}", path),
        Err(err) => println!("failure: {}", pretty_error(&err)),
    }
    // println!("{:?}, {:?}", err.cause(), err.backtrace())
}

#[derive(Debug, Fail)]
pub struct Error {
    ctx: Context<ErrorKind>,
}

impl Error {
    pub fn io(err: io::Error) -> Error {
        Error::from(ErrorKind::IO(err.to_string()))
    }

    pub fn path<P: AsRef<Path>>(path: P) -> Error {
        let kind = ErrorKind::Path(path.as_ref().to_path_buf());
        Error::from(kind)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.ctx.fmt(f)
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        Error::from(Context::new(kind))
    }
}

impl From<Context<ErrorKind>> for Error {
    fn from(ctx: Context<ErrorKind>) -> Error {
        Error { ctx }
    }
}

#[derive(Debug, Fail)]
pub enum ErrorKind {
    Path(PathBuf),
    IO(String),
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ErrorKind::Path(ref path) => write!(f, "path: {}", path.display()),
            ErrorKind::IO(ref msg) => write!(f, "IO error: {}", msg),
        }
    }
}

/// Return a prettily formatted error, including its entire causal chain.
fn pretty_error(err: &failure::Error) -> String {
    let mut pretty = err.to_string();
    let mut prev = err.as_fail();
    while let Some(next) = prev.cause() {
        pretty.push_str(": ");
        pretty.push_str(&next.to_string());
        prev = next;
    }

    pretty.push_str(&format!("{}", err.backtrace()));
    pretty
}

pub fn resolve(path: PathBuf) -> Result<PathBuf, failure::Error> {
    let path = env::current_dir()
        .map(|mut p| {
            p.push(path.to_path_buf());
            p
        })
        .map_err(Error::io)
        .context("failed to get current dir")?;

    let path = path
        .canonicalize()
        .map_err(Error::io)
        .with_context(|_| Error::path(path))?;

    Ok(path)
}
