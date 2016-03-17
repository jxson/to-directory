use std::path::PathBuf;
use types::{ToResult, ToError, Bookmark, Bookmarks};

// TODO implement display
pub struct Store {
    db: PathBuf,
    // bookmarks: Cell<Bookmarks>, // Maybe?
}

impl Store {
    pub fn new(db: PathBuf) -> Store {
        return Store{ db: db };
    }

    fn create(&self) -> ToResult<()> {
        return Ok(());
    }

    pub fn all(&self) -> ToResult<Bookmarks> {
        let collection = Bookmarks::new();
        return Ok(collection);
    }

    pub fn put(&self, name: String, directory: PathBuf) -> ToResult<()> {
        println!("PUT: {:?} {:?}", name, directory);

        let mut collection = match self.all() {
            Ok(value) => value,
            Err(err) => panic!(err),
        };

        let key = name.clone();
        let value = Bookmark::new(name, directory);
        collection.insert(key, value);

        println!("collection {:?}", collection);

        if let Err(err) = self.save(collection) {
            print!("Error on put");
        }

        return Ok(());
    }

    fn save(&self, bookmarks: Bookmarks) -> ToResult<()> {
        return Ok(());
    }
}
