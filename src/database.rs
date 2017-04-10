use std::path::PathBuf;
use std::collections::BTreeMap;
use std::collections::btree_map::Iter;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::{BufReader, BufWriter};
use bincode::{deserialize_from, serialize_into, Infinite};
use errors::*;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Bookmark {
    pub name: String,
    pub directory: PathBuf,
    created_at: u64,
    pub updated_at: u64,
    accessed_at: Option<u64>,
}

pub type Bookmarks = BTreeMap<String, Bookmark>;

impl Bookmark {
    pub fn new(name: String, directory: PathBuf) -> Bookmark {
        return Bookmark {
            name: name,
            directory: directory,
            created_at: ::now(),
            updated_at: ::now(),
            accessed_at: None,
        };
    }
}

#[derive(Debug)]
pub struct Database {
    location: PathBuf,
    bookmarks: Bookmarks,
}

impl Database {
    pub fn open(mut path: PathBuf) -> Result<Database> {
        if !path.ends_with("db") { path.push("db"); }

        let bookmarks = match File::open(&path) {
            Ok(file) => try!(hydrate(file)),
            Err(ref err) if notfound(err) => Bookmarks::new(),
            Err(err) => bail!(err),
        };

        let db = Database::new(path, bookmarks);
        Ok(db)
    }

    fn new(location: PathBuf, bookmarks: Bookmarks) -> Database {
        Database {
            location: location,
            bookmarks: bookmarks,
        }
    }

    pub fn get(&self, key: String) -> Result<&Bookmark> {
        let bookmark = match self.bookmarks.get(&key) {
            Some(bookmark) => bookmark,
            None => bail!(ErrorKind::BookmarkNotFound(key)),
        };

        Ok(bookmark)
    }

    // TODO: add a check to verify the db is open.
    // TODO: add check to verify value is a valid directory.
    pub fn put(&mut self, key: String, value: PathBuf) -> Result<()> {
        if self.bookmarks.contains_key(&key) {
            self.update(key, value)
        } else {
            self.create(key, value)
        }
    }

    pub fn delete(&mut self, key: String) -> Result<()> {
        match self.bookmarks.remove(&key) {
            None => bail!(ErrorKind::BookmarkNotFound(key)),
            _ => {},
        };

        try!(self.close());
        Ok(())
    }

    fn update(&mut self, key: String, path: PathBuf) -> Result<()>{
        match self.bookmarks.get_mut(&key) {
            Some(bookmark) => {
                bookmark.directory = path;
                bookmark.updated_at = ::now();
            }
            None => bail!(ErrorKind::BookmarkNotFound(key))
        }

        try!(self.close());
        Ok(())
    }

    fn create(&mut self, key: String, value: PathBuf) -> Result<()> {
        let bookmark = Bookmark::new(key.clone(), value);
        self.bookmarks.insert(key, bookmark);

        try!(self.close());
        Ok(())
    }

    pub fn list<'a>(&'a self) -> Iter<'a, String, Bookmark> {
        return self.bookmarks.iter();
    }

    fn close(&self) -> Result<()> {
        let mut options = OpenOptions::new();
                options.write(true);

        let file = match options.open(&self.location) {
            Ok(file) => file,
            Err(_) => {
                options.create(true);
                try!(options.open(&self.location))
            }
        };

        try!(dehydrate(file, &self.bookmarks));
        Ok(())
    }
}

fn notfound(err: &io::Error) -> bool {
    err.kind() == io::ErrorKind::NotFound
}

fn hydrate(file: File) -> Result<Bookmarks> {
    let mut reader = BufReader::new(file);
    let bookmarks: Bookmarks = try!(deserialize_from(&mut reader, Infinite));
    Ok(bookmarks)
}

fn dehydrate(file: File, bookmarks: &Bookmarks) -> Result<()> {
    let mut writer = BufWriter::new(file);

    try!(serialize_into(&mut writer, &bookmarks, Infinite));
    Ok(())
}
