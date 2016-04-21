use std::collections::BTreeMap;
use std::io;
use std::path::PathBuf;

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

#[derive(Debug, RustcEncodable, RustcDecodable, PartialEq)]
pub struct Bookmark {
    pub name: String,
    pub directory: PathBuf, /* pub created_at: DateTime<UTC>,
                             * pub updted_at: DateTime<UTC>,
                             * pub last_accessed_at: DateTime<UTC>, */
}

impl Bookmark {
    pub fn new(name: String, directory: PathBuf) -> Bookmark {
        return Bookmark {
            name: name,
            directory: directory,
        };
    }
}

pub type Bookmarks = BTreeMap<String, Bookmark>;
