use bincode;
pub use failure::ResultExt;
use failure::{Context, Fail};
use std::path::{Path, PathBuf};
use std::{fmt, io, result};

pub type Result<T> = result::Result<T, failure::Error>;

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

    pub fn not_found(key: String) -> Error {
        let kind = ErrorKind::NotFound(key);
        Error::from(kind)
    }

    pub fn bincode(err: bincode::Error) -> Error {
        let kind = ErrorKind::Bincode(err.to_string());
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
    NotFound(String),
    Bincode(String),
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ErrorKind::Path(ref path) => write!(f, "path: {}", path.display()),
            ErrorKind::IO(ref msg) => write!(f, "IO error: {}", msg),
            ErrorKind::NotFound(ref bookmark) => write!(f, "bookmark not found: {}", bookmark),
            ErrorKind::Bincode(ref msg) => write!(f, "bincode error: {}", msg),
        }
    }
}

pub fn pretty_error(err: &failure::Error) -> String {
    let mut pretty = format!("\n => {}", err.to_string());
    let mut prev = err.as_fail();
    while let Some(next) = prev.cause() {
        pretty.push_str("\n => ");
        pretty.push_str(&next.to_string());
        prev = next;
    }

    pretty.push_str(&format!("\n\n{}", err.backtrace()));

    pretty
}
