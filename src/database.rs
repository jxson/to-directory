use error::{ToResult, ToError};
use std::cell::Cell;
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
    // TODO: add check to verify value is a valid directory.
    pub fn put(mut self, key: String, value: PathBuf) -> ToResult<()> {
        println!("put: {:?}: {:?}", key, value);

        // let bookmark = match self.get(key) {
        //     Some(bookmark) => {
        //         Bookmark::new(key.clone(), value);
        //     },
        //     None => {
        //         Bookmark::new(key.clone(), value);
        //     }
        // };

        // update a key, guarding against the key possibly not being set
        // let stat = player_stats.entry("attack").or_insert(100);
        // *stat += random_stat_buff();

        let mut bookmark =  match self.get(&key) {
            Some(bookmark) => bookmark.to_owned(),
            None => {
                panic!("omg");
            },
        };



        println!("bookmark: {:?}", bookmark);

        bookmark.directory = value;

        println!("bookmark: {:?}", bookmark);


        // let bookmark = Bookmark::new(key.clone(), value);


        match self.close() {
            Ok(value) => return Ok(value),
            Err(err) => panic!("Failed to close: {:?}", err),
        };
    }

    pub fn get(&self, key: &String) -> Option<&Bookmark> {
        return self.bookmarks.get(key);
    }

    fn close(&self) -> ToResult<()> {
        // return Ok(());
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

#[derive(Copy, Debug, RustcEncodable, RustcDecodable, PartialEq)]
pub struct Bookmark {
    pub name: String,
    pub directory: Cell<PathBuf>,
    created_at: i64,
    updted_at: i64,
    accessed_at: Option<i64>,
}

impl Bookmark {
    pub fn new(name: String, directory: PathBuf) -> Bookmark {
        let now = 0;

        return Bookmark {
            name: name,
            directory: Cell::new(directory),
            created_at: now,
            updted_at: now,
            accessed_at: None,
        };
    }
}
