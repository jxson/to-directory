use std::path::{PathBuf, Components};
use types::{ToResult, ToError, Bookmark, Bookmarks};
use std::fs::{File};
use std::io::{BufReader, ErrorKind};
use dir::{mkdirp};

// TODO implement display
pub struct Store {
    db: PathBuf,
    // bookmarks: Cell<Bookmarks>, // Maybe?
}

impl Store {
    pub fn new(directory: PathBuf) -> Store {
        let mut db = PathBuf::from(directory);
                db.push("db.bin");

        println!("db {:?}", db);
        return Store{ db: db };
    }

    pub fn all(&self) -> ToResult<Bookmarks> {
        let collection = match self.read() {
            Ok(value) => value,
            Err(err) => panic!(err),
        };

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

        if let Err(err) = self.write(collection) {
            print!("Error on put");
        }

        return Ok(());
    }

    fn create(&self) -> ToResult<()> {
        return Ok(());
    }

    fn read(&self) -> ToResult<Bookmarks> {
        let file = match bootstrap(&self.db) {
            Ok(value) => value,
            Err(err) => return Err(err),
        };

        // let mut reader = BufReader::new(file);

        let bookmarks = Bookmarks::new();

        return Ok(bookmarks);
    }

    fn write(&self, bookmarks: Bookmarks) -> ToResult<()> {
        return Ok(());
    }
}

fn bootstrap(db: &PathBuf) -> ToResult<File> {
    let directory = match db.parent() {
        Some(value) => value,
        None => panic!("db cannot be a idrectory."),
    };

    if let Err(err) = mkdirp(directory) {
        return Err(err);
    }

    match File::create(&db) {
        Ok(_) => {},
        Err(ref err) if err.kind() == ErrorKind::AlreadyExists => {},
        Err(err) => return Err(ToError::Io(err)),
    };

    let file = match File::open(db) {
        Ok(value) => value,
        Err(err) => return Err(ToError::Io(err)),
    };

    return Ok(file);
}

fn open(file: PathBuf) -> ToResult<File> {
    match File::open(file) {
        Ok(value) => return Ok(value),
        Err(err) => return Err(ToError::Io(err)),
    }
}
