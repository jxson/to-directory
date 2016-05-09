// use std::io::Error;
use clap::{App, ArgMatches};
extern crate env_logger;


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
    let yaml = load_yaml!("cli.yml");
    let app = App::from_yaml(yaml);
    let matches = app.get_matches();

    return get(matches);
}

pub fn get_request_from(args: Vec<&str>) -> ToResult<Request> {
    let yaml = load_yaml!("cli.yml");
    let app = App::from_yaml(yaml);
    let matches = app.get_matches_from(args);

    return get(matches);
}

fn get(matches: ArgMatches) -> ToResult<Request> {
    info!("cli parsing matches: {:?}", matches);

    let request = Request::new();
    return Ok(request);
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate env_logger;

    #[test]
    fn basic() {
        let _ = env_logger::init();
        let request = get_request();
        assert!(request.is_ok());
    }

    #[test]
    fn basic_from_vec() -> () {
        let _ = env_logger::init();
        let args = vec!["foo", "bar"];
        let request = get_request_from(args);
        assert!(request.is_ok());
    }
}
