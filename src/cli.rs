use std::path::PathBuf;

pub struct Request {
    pub name: String,
    pub directory: PathBuf,
}

impl Request {
    fn new(name: &str, directory: &PathBuf) -> Request {
        Request{ name: "foo".to_string(), directory: PathBuf::from(".")}
    }
}
use std::fmt;

impl fmt::Debug for Request {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Request{{ name: {:?}, directory: {:?} }}", self.name, self.directory)
    }
}
