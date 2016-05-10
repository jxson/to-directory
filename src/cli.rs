use clap::{App, ArgMatches};
use std::path::{PathBuf};
use log;

extern crate env_logger;


pub type ToResult<T> = Result<T, ToError>;

#[derive(Debug)]
pub enum ToError {
    Log(log::SetLoggerError),
}

// TODO: add custom displays for these errors.
// * SEE: https://jadpole.github.io/rust/many-error-types
// * SEE: http://lucumr.pocoo.org/2014/11/6/error-handling-in-rust/
impl From<log::SetLoggerError> for ToError {
    fn from(err: log::SetLoggerError) -> ToError {
        ToError::Log(err)
    }
}

#[derive(Debug)]
pub struct Request {
    pub name: String,
    pub directory: PathBuf,
    pub action: Action,
}

impl Request {
    fn new(action: Action) -> Request {
        Request {
            name: String::from("foo"),
            directory: PathBuf::from("bar"),
            action: action,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Action {
    Get,
    Put,
    Delete,
    List,
    Last,
    ChangeDirectory,
}

pub fn get_request() -> ToResult<Request> {
    let yaml = load_yaml!("cli.yml");
    let app = App::from_yaml(yaml);
    let matches = app.get_matches();

    return get(matches);
}

pub fn get(matches: ArgMatches) -> ToResult<Request> {
    info!("building request from clap::{:?}", matches);

    let request = Request::new(Action::ChangeDirectory);
    return Ok(request);
}

#[cfg(test)]
mod tests {
    extern crate env_logger;
    extern crate clap;

    use super::*;
    use std::path::{PathBuf};

    fn run(mut args: Vec<&str>) -> Request {
        let logger_result = env_logger::init();
        assert!(logger_result.is_ok());

        args.insert(0, "to");

        let yaml = load_yaml!("cli.yml");
        let app = clap::App::from_yaml(yaml);
        let matches = app.get_matches_from(args);

        let result = get(matches);
        assert!(result.is_ok());

        return result.unwrap();
    }

    #[test]
    fn basic() {
        let request = get_request();
        assert!(request.is_ok());
    }

    #[test]
    fn to_name() -> () {
        let args = vec!["foo"];
        let request = run(args);

        assert_eq!(request.action, Action::ChangeDirectory);
        assert_eq!(request.name, "foo");
        assert_eq!(request.directory, PathBuf::from("bar"));
    }
}
