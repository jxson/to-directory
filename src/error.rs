use log;
use std::io;

pub type ToResult<T> = Result<T, ToError>;

#[derive(Debug)]
pub enum ToError {
    Log(log::SetLoggerError),
    Io(io::Error),
}

// TODO: add custom displays for these errors.
// * SEE: https://jadpole.github.io/rust/many-error-types
// * SEE: http://lucumr.pocoo.org/2014/11/6/error-handling-in-rust/
impl From<log::SetLoggerError> for ToError {
    fn from(err: log::SetLoggerError) -> ToError {
        ToError::Log(err)
    }
}

impl From<io::Error> for ToError {
    fn from(err: io::Error) -> ToError {
        ToError::Io(err)
    }
}
