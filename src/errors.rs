use failure::{Backtrace, Context, Fail, ResultExt};
use std::env;
use std::fmt;
use std::fmt::Display;
use std::io;
use std::path::{Path, PathBuf};

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
pub fn format_chain(err: &failure::Error) -> String {
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
