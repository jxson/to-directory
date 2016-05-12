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
    fn new(name: &str, directory: PathBuf, action: Action) -> Request {
        Request {
            name: String::from(name),
            directory: directory,
            action: action,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Action {
    Get,
    Put,
    List,
    Delete,
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
    info!("Building CLI request");

    let pathname = match matches.value_of("DIRECTORY") {
        Some(value) => value,
        None => "",
    };

    let directory = try!(dir::resolve(pathname));
    let basename = dir::basename(&directory).expect("TODO: handle this case");
    let name = matches.value_of("NAME").unwrap_or(basename.as_str());
    let request = Request::new(name, directory, Action::ChangeDirectory);

    return Ok(request);
}

#[cfg(test)]
mod tests {
    extern crate env_logger;
    extern crate clap;

    use super::*;
    use std::env;
    use std::path::{PathBuf};

    fn run(mut args: Vec<&str>) -> Request {
        // Logger might fail if it is already initialized.
        // let logger_result = env_logger::init();
        // assert!(logger_result.is_ok());
        let _ = env_logger::init();

        args.insert(0, "to");

        let yaml = load_yaml!("cli.yml");
        let app = clap::App::from_yaml(yaml);
        let matches = app.get_matches_from(args);

        let request = get(matches).expect("should not fail");
        return request;
    }

    #[test]
    fn basic() {
        let result = get_request();
        assert!(result.is_ok());
    }

    #[test]
    fn to_name() {
        let cwd = env::current_dir().expect("should not fail");
        let args = vec!["foo"];
        let request = run(args);

        assert_eq!(request.action, Action::ChangeDirectory);
        assert_eq!(request.name, "foo");
        assert_eq!(request.directory, cwd);
    }
}
