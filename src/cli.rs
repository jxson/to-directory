use clap::{App, ArgMatches};
use std::path::{PathBuf};

use dir;
use error::{ToResult};

#[derive(Debug)]
pub struct Request {
    pub name: String,
    pub directory: PathBuf,
    pub action: Action,
    pub verbose: bool,
}

impl Request {
    fn new(name: &str, directory: PathBuf, action: Action, verbose: bool) -> Request {
        Request {
            name: String::from(name),
            directory: directory,
            action: action,
            verbose: verbose,
        }
    }

    pub fn get() -> ToResult<Request> {
        let yaml = load_yaml!("cli.yml");
        let app = App::from_yaml(yaml).version(crate_version!());

        return Request::from(app);
    }

    pub fn from(app: App) -> ToResult<Request> {
        let _app = app.clone();
        let matches = app.get_matches();
        let pathname = matches.value_of("DIRECTORY").unwrap_or("");
        let directory = try!(dir::resolve(pathname));

        let basename = dir::basename(&directory).expect("TODO: handle this case");
        let name = matches.value_of("NAME").unwrap_or(basename.as_str());

        let action = Action::from(&matches);
        match action {
            Action::Help => {
                _app.print_help();
                panic!("");
            },
            _ => {},
        }
        let verbose = match matches.occurrences_of("verbose") {
            0 => false,
            _ => true,
        };

        let request = Request::new(name, directory, action, verbose);

        return Ok(request);
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
    Help,
}

impl Action {
    pub fn from(matches: &ArgMatches) -> Action {
        let (get, put, list, delete, last) = (
            matches.is_present("get"),
            matches.is_present("put"),
            matches.is_present("list"),
            matches.is_present("delete"),
            matches.value_of("NAME") == Some("-"),
        );

        let action = match (get, put, list, delete, last) {
            (true, _, _, _, _) => Action::Get,
            (_, true, _, _, _) => Action::Put,
            (_, _, true, _, _) => Action::List,
            (_, _, _, true, _) => Action::Delete,
            (_, _, _, _, true) => Action::Last,
            _ => {
                if matches.value_of("NAME").is_some() {
                    Action::ChangeDirectory
                } else {
                    Action::Help
                }
            },
        };

        return action;
    }
}

#[cfg(test)]
mod tests {
    extern crate clap;

    use super::*;
    use std::env;

    fn run(mut args: Vec<&str>) -> Request {
        args.insert(0, "to");

        let yaml = load_yaml!("cli.yml");
        let app = clap::App::from_yaml(yaml);
        let matches = app.get_matches_from(args);

        let request = Request::from(matches).expect("should not fail");
        return request;
    }

    #[test]
    fn basic() {
        let result = Request::get();
        assert!(result.is_ok());
    }

    #[test]
    fn test_name() {
        let cwd = env::current_dir().expect("should not fail");
        let args = vec!["foo"];
        let request = run(args);

        assert_eq!(request.action, Action::ChangeDirectory);
        assert_eq!(request.name, "foo");
        assert_eq!(request.directory, cwd);
    }

    #[test]
    fn no_flags() {
        let request = run(vec![]);
        assert_eq!(request.action, Action::ChangeDirectory);
    }

    #[test]
    fn flag_info() {
        let request = run(vec!["--info"]);
        assert_eq!(request.action, Action::Get);

        let request = run(vec!["-i"]);
        assert_eq!(request.action, Action::Get);
    }

    #[test]
    fn flag_save() {
        let request = run(vec!["--save"]);
        assert_eq!(request.action, Action::Put);

        let request = run(vec!["-s"]);
        assert_eq!(request.action, Action::Put);
    }

    #[test]
    fn flag_list() {
        let request = run(vec!["--list"]);
        assert_eq!(request.action, Action::List);

        let request = run(vec!["-l"]);
        assert_eq!(request.action, Action::List);
    }

    #[test]
    fn flag_delete() {
        let request = run(vec!["--delete"]);
        assert_eq!(request.action, Action::Delete);

        let request = run(vec!["-d"]);
        assert_eq!(request.action, Action::Delete);
    }

    #[test]
    fn flag_verbose() {
        let request = run(vec![]);
        assert_eq!(request.verbose, false);

        let request = run(vec!["--verbose"]);
        assert_eq!(request.verbose, true);

        let request = run(vec!["-v"]);
        assert_eq!(request.verbose, true);
    }

    #[test]
    fn last() {
        let request = run(vec!["-"]);
        assert_eq!(request.action, Action::Last);
    }
}
