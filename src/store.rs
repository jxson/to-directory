use std::path::PathBuf;
use types::{ToResult, ToError, Bookmark};

// TODO implement display
pub struct Store {
    db: PathBuf,
}

impl Store {
    pub fn new(dir: PathBuf) -> Store {
        let mut db = PathBuf::from(dir);
                db.push(".to");

        return Store{ db: db };
    }

    pub fn put(&self, name: String, directory: PathBuf) -> ToResult<()> {
        println!("PUT: {:?} {:?}", name, directory);
        return Ok(());
    }
}
