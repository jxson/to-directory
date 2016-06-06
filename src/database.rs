use error::{ToResult, ToError};
use std::path::PathBuf;
use std::collections::BTreeMap;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, ErrorKind};
use bincode::SizeLimit::Infinite;
use bincode::rustc_serialize::{encode_into, decode_from};

pub type Bookmarks = BTreeMap<String, Bookmark>;

#[derive(Debug)]
pub struct Database {
    location: PathBuf,
    bookmarks: Bookmarks,
}

impl Database {
    fn new(location: PathBuf, bookmarks: Bookmarks) -> Database {
        Database {
            location: location,
            bookmarks: bookmarks,
        }
    }

    pub fn open(directory: PathBuf) -> ToResult<Database> {
        let bookmarks = match File::open(&directory) {
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

        return Ok(Database::new(directory, bookmarks));
    }

    // TODO: add a check to verify the db is open.
    pub fn put(&mut self, key: String, value: PathBuf) -> ToResult<()> {
        let value = Bookmark::new(key.clone(), value);
        self.bookmarks.insert(key, value);

        match self.close() {
            Ok(value) => return Ok(value),
            Err(err) => panic!("Failed to close: {:?}", err),
        };
    }

    fn close(&self) -> ToResult<()> {
        let mut options = OpenOptions::new();
                options.write(true);

        let file = match options.open(&self.location) {
            Ok(file) => file,
            // Does not exist, create it.
            Err(_) => {
                options.create(true);
                try!(options.open(&self.location))
            }
        };

        let mut writer = BufWriter::new(file);
        match encode_into(&self.bookmarks, &mut writer, Infinite) {
            Ok(_) => println!("successful encode"),
            Err(err) => panic!("ERROR ECODING: {:?}", err),
        }

        return Ok(());
    }
}

#[derive(Debug, RustcEncodable, RustcDecodable, PartialEq)]
pub struct Bookmark {
    pub name: String,
    pub directory: PathBuf,
}

impl Bookmark {
    pub fn new(name: String, directory: PathBuf) -> Bookmark {
        return Bookmark {
            name: name,
            directory: directory,
        };
    }
}
