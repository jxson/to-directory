use bincode;
use failure::{Backtrace, Context, Fail};
use std::path::{Path, PathBuf};
use std::{fmt, io, result};

pub type Result<T> = result::Result<T, Error>;

#[derive(Fail, Debug)]
pub struct Error {
    ctx: Context<ErrorKind>,
}

impl Error {
    pub fn kind(&self) -> &ErrorKind {
        self.ctx.get_context()
    }

    pub fn io(err: io::Error) -> Error {
        Error::from(ErrorKind::IO(err.to_string()))
    }

    pub fn path<P: AsRef<Path>>(path: P) -> Error {
        let kind = ErrorKind::Path(path.as_ref().to_path_buf());
        Error::from(kind)
    }

    pub fn config() -> Error {
        let kind = ErrorKind::Config;
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

    fn backtrace(&self) -> Option<&Backtrace> {
        self.ctx.backtrace()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.ctx.fmt(f)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        let kind = ErrorKind::IO(err.to_string());
        Error::from(Context::new(kind))
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

#[derive(Fail, Debug)]
pub enum ErrorKind {
    Path(PathBuf),
    IO(String),
    NotFound(String),
    Config,
    Bincode(String),
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ErrorKind::Path(ref path) => write!(f, "path: {}", path.display()),
            ErrorKind::IO(ref msg) => write!(f, "IO error: {}", msg),
            ErrorKind::NotFound(ref bookmark) => write!(f, "bookmark not found: {}", bookmark),
            ErrorKind::Config => write!(f, "failed to locat config"),
            ErrorKind::Bincode(ref msg) => write!(f, "bincode error: {}", msg),
        }
    }
}

/// Return a prettily formatted error, including its entire causal chain.
pub fn format_chain(err: &Error) -> String {
    let mut pretty = err.to_string();
    let mut prev = err.cause().unwrap();
    while let Some(next) = prev.cause() {
        pretty.push_str(": ");
        pretty.push_str(&next.to_string());
        prev = next;
    }

    if let Some(backtrace) = err.backtrace() {
        pretty.push_str(&format!("{}", backtrace));
    }

    pretty
}
