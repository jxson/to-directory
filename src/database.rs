use std::path::PathBuf;
use types::{ToResult, ToError, Bookmark, Bookmarks};
use dir::mkdirp;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, ErrorKind};
use bincode::SizeLimit::Infinite;
use bincode::rustc_serialize::{encode_into, decode_from};
use std::string::String;
use std::cell::Cell;

// use std::path::{PathBuf, Components};

#[derive(Debug)]
pub struct Database {
    location: PathBuf,
    // bookmarks: Cell<Bookmarks>,
    bookmarks: Bookmarks,
}

//     pub fn all(&self) -> ToResult<Bookmarks> {
//         let collection = match self.read() {
//             Ok(value) => value,
//             Err(err) => panic!(err),
//         };
//
//         return Ok(collection);
//     }

impl Database {
    fn new(location: PathBuf, bookmarks: Bookmarks) -> Database {
        Database {
            location: location,
            bookmarks: bookmarks,
        }
    }

    pub fn open(directory: PathBuf) -> ToResult<Database> {
        println!("DB open");

        if let Some(extension) = directory.extension() {
            panic!("path is required to be a directory");
        }

        if let Err(err) = mkdirp(&directory) {
            return Err(err);
        }

        let mut location = PathBuf::from(directory);
        location.push("db.bin");

        let bookmarks = match File::open(&location) {
            Ok(file) => {
                let mut reader = BufReader::new(file);
                let bookmarks: Bookmarks = match decode_from(&mut reader, Infinite) {
                    Ok(value) => value,
                    Err(err) => panic!("ERROR DECODING: {:?}", err),
                };

                bookmarks
            }
            Err(ref err) if err.kind() == ErrorKind::NotFound => Bookmarks::new(),
            Err(err) => return Err(ToError::Io(err)),
        };

        let db = Database::new(location, bookmarks);
        return Ok(db);
    }

    pub fn put(&mut self, key: String, value: PathBuf) -> ToResult<()> {
        println!("db.put({:?}, {:?})", key, value);
        // Check that the db has been opened.

        let value = Bookmark::new(key.clone(), value);
        self.bookmarks.insert(key, value);
        println!("bookmarks {:?}", self.bookmarks);

        match self.close() {
            Ok(value) => return Ok(value),
            Err(err) => panic!("Failed to close: {:?}", err),
        };
    }

    pub fn get(&mut self, key: String) -> ToResult<&Bookmark> {
        match self.bookmarks.get(&key) {
            Some(value) => return Ok(value),
            None => panic!("NOT FOUND"),
        }
    }

    fn close(&self) -> ToResult<()> {
        println!("CLOSING");
        // let path = PathBuf::from(&self.location);
        let mut options = OpenOptions::new();
        options.write(true);
        let file = match options.open(&self.location) {
            Ok(file) => file,
            // Does not exist, create it.
            Err(err) => {
                options.create(true);
                try!(options.open(&self.location))
            }
        };

        println!("file {:?}", file);

        let mut writer = BufWriter::new(file);
        match encode_into(&self.bookmarks, &mut writer, Infinite) {
            Ok(_) => println!("successful encode"),
            Err(err) => panic!("ERROR ECODING: {:?}", err),
        }

        println!("closed {:?}", self.location);

        return Ok(());
    }
}
