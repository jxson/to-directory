use clap::{App, ArgMatches};
use std::path::{PathBuf};

use dir;
use error::{ToResult};

extern crate env_logger;

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

    let pathname = match matches.value_of("directory") {
        Some(value) => value,
        None => "",
    };

    let directory = try!(dir::resolve(pathname));

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
        let result = get_request();
        assert!(result.is_ok());
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
