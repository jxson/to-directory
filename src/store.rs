use std::path::{PathBuf, Components};
use types::{ToResult, ToError, Bookmark, Bookmarks};
use std::fs::{File};
use std::io::{BufReader, ErrorKind};
use dir::{mkdirp};

// TODO implement display
pub struct Store {
    location: PathBuf,
    // bookmarks: Cell<Bookmarks>, // Maybe?
}

impl Store {
    pub fn new(location: PathBuf) -> Store {
        let mut location = PathBuf::from(location);
                location.push("db.bin");

        println!("location {:?}", location);
        return Store{ location: location };
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

    fn read(&self) -> ToResult<Bookmarks> {
        let file = match open(&self.location) {
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

fn open(location: &PathBuf) -> ToResult<File> {
    let directory = match location.parent() {
        Some(value) => value,
        None => panic!("location cannot be a idrectory."),
    };

    if let Err(err) = mkdirp(directory) {
        return Err(err);
    }

    match File::create(&location) {
        Ok(_) => {},
        Err(ref err) if err.kind() == ErrorKind::AlreadyExists => {},
        Err(err) => return Err(ToError::Io(err)),
    };

    let file = match File::open(location) {
        Ok(value) => value,
        Err(err) => return Err(ToError::Io(err)),
    };

    return Ok(file);
}
