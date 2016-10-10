use std::io::Error as IoError;
use std::error::Error;
use std::fmt;

use log::SetLoggerError;

pub type ToResult<T> = Result<T, ToError>;

#[derive(Debug)]
pub enum ToError {
    Io(IoError),
    // TODO(jxson): Set this up so SetLoggerError is passive in test.
    SetLoggerError(SetLoggerError),
    UnknownHomeDirectory,
    BookmarkNotFound,
}

impl fmt::Display for ToError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ToError::Io(ref err) => err.fmt(f),
            ToError::SetLoggerError(ref err) => err.fmt(f),
            ToError::UnknownHomeDirectory => write!(f, "Unable to locate $HOME directory."),
            ToError::BookmarkNotFound => write!(f, "Bookmark not found."),
        }
    }
}

impl Error for ToError {
    fn description(&self) -> &str {
        match *self {
            ToError::Io(ref err) => err.description(),
            ToError::SetLoggerError(ref err) => err.description(),
            ToError::UnknownHomeDirectory => "$HOME not set",
            ToError::BookmarkNotFound => "",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            ToError::Io(ref err) => Some(err),
            ToError::SetLoggerError(ref err) => Some(err),
            ToError::UnknownHomeDirectory => None,
            ToError::BookmarkNotFound => None,
        }
    }
}

impl From<IoError> for ToError {
    fn from(err: IoError) -> ToError {
        ToError::Io(err)
    }
}

impl From<SetLoggerError> for ToError {
    fn from(err: SetLoggerError) -> ToError {
        ToError::SetLoggerError(err)
    }
}
