pub mod cli;

use std::io;

pub type ToResult<T> = Result<T, ToError>;

// SEE: http://lucumr.pocoo.org/2014/11/6/error-handling-in-rust/
#[derive(Debug)]
pub enum ToError {
    Io(io::Error),

}

impl From<io::Error> for ToError {
    fn from(err: io::Error) -> ToError {
        ToError::Io(err)
    }
}
