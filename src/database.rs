use time;
use error::{ToResult, ToError};
use std::path::PathBuf;
use std::collections::BTreeMap;
use std::collections::btree_map::Iter;
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
            },
            Err(ref err) if err.kind() == ErrorKind::NotFound => Bookmarks::new(),
            Err(err) => return Err(ToError::Io(err)),
        };

        return Ok(Database::new(directory, bookmarks));
    }

    // TODO: add a check to verify the db is open.
    // TODO: add check to verify value is a valid directory.
    pub fn put(&mut self, key: String, value: PathBuf) -> ToResult<()> {
        if self.bookmarks.contains_key(&key) {
            if let Some(bookmark) = self.bookmarks.get_mut(&key) {
                bookmark.directory = value;
                bookmark.updated_at = now();
            }
        } else {
            let bookmark = Bookmark::new(key.clone(), value);
            self.bookmarks.insert(key, bookmark);
        }

        match self.close() {
            Ok(value) => return Ok(value),
            Err(err) => panic!("Failed to close: {:?}", err),
        };
    }

    pub fn get(&self, key: &String) -> Option<&Bookmark> {
        return self.bookmarks.get(key);
    }

    pub fn delete(&mut self, key: String) -> ToResult<()> {
        match self.bookmarks.remove(&key) {
            None => panic!("nothing to delete."),
            _ => {},
        }

        match self.close() {
            Ok(value) => return Ok(value),
            Err(err) => panic!("Failed to close: {:?}", err),
        };
    }

    pub fn all<'a>(&'a self) -> Iter<'a, String, Bookmark> {
        return self.bookmarks.iter();
    }

    fn close(&self) -> ToResult<()> {
        let mut options = OpenOptions::new();
                options.write(true);

        let file = match options.open(&self.location) {
            Ok(file) => file,
            Err(_) => {
                options.create(true);
                try!(options.open(&self.location))
            }
        };

        let mut writer = BufWriter::new(file);
        match encode_into(&self.bookmarks, &mut writer, Infinite) {
            Ok(_) => {},
            Err(err) => panic!("ERROR ECODING: {:?}", err),
        }

        return Ok(());
    }
}

#[derive(Debug, RustcEncodable, RustcDecodable, PartialEq)]
pub struct Bookmark {
    pub name: String,
    pub directory: PathBuf,
    created_at: i64,
    pub updated_at: i64,
    accessed_at: Option<i64>,
}

impl Bookmark {
    pub fn new(name: String, directory: PathBuf) -> Bookmark {
        return Bookmark {
            name: name,
            directory: directory,
            created_at: now(),
            updated_at: now(),
            accessed_at: None,
        };
    }
}

fn now() -> i64 {
    return time::now_utc().to_timespec().sec;
}
