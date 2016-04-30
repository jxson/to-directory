// use std::io::Error;

use std::env;
use std::env::ArgsOs;
use clap;
use clap::{App, Arg};


pub type ToResult<T> = Result<T, ToError>;

#[derive(Debug)]
pub enum ToError {
    // Io(Error)
}

#[derive(Debug)]
pub struct Request {

}

impl Request {
    fn new() -> Request {
        Request {}
    }
}

pub fn get_request() -> ToResult<Request> {
    let args = env::args_os();
    return get_request_from(args);

}

pub fn get_request_from(args: ArgsOs) -> ToResult<Request> {
    let yaml = load_yaml!("cli.yml");
    let app = App::from_yaml(yaml);
    let matches = app.get_matches_from(args);

    let request = Request::new();
    return Ok(request);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let request = get_request();
        assert!(request.is_ok());
    }

    #[test]
    fn basic_from_vec() -> ToResult<Request> {
        let args = vec!["foo", "bar"];
        let request = get_request_from(args);
        assert!(request.is_ok());
    }
}
