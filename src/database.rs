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
    pub created_at: u64,
    pub updated_at: u64,
    pub last_access: Option<u64>,
    pub count: i64,
}

pub type Bookmarks = BTreeMap<String, Bookmark>;

impl Bookmark {
    pub fn new(name: String, directory: PathBuf) -> Bookmark {
        let timestamp = ::now();

        Bookmark {
            name: name,
            directory: directory,
            created_at: timestamp,
            updated_at: timestamp,
            last_access: None,
            count: 0,
        }
    }
}

#[derive(Debug)]
pub struct Database {
    pub location: PathBuf,
    bookmarks: Bookmarks,
}

impl Database {
    pub fn open(mut path: PathBuf) -> Result<Database> {
        if !path.ends_with("db") {
            path.push("db");
        }

        let bookmarks = match File::open(&path) {
            Ok(file) => try!(hydrate(file)),
            Err(ref err) if notfound(err) => Bookmarks::new(),
            Err(_) => bail!(ErrorKind::DBOpenError(path)),
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

    pub fn get(&self, key: &str) -> Option<&Bookmark> {
        self.bookmarks.get(key)
    }

    pub fn get_path(&mut self, key: &str) -> Result<PathBuf> {
        let path: PathBuf;
        match self.bookmarks.get_mut(key) {
            Some(bookmark) => {
                bookmark.last_access = Some(::now());
                path = bookmark.directory.clone();
            }
            None => bail!(ErrorKind::BookmarkNotFound(key.to_string())),
        };

        try!(self.close());
        Ok(path)
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
        if self.bookmarks.remove(&key).is_none() {
            bail!(ErrorKind::BookmarkNotFound(key));
        }

        try!(self.close());
        Ok(())
    }

    fn update(&mut self, key: String, path: PathBuf) -> Result<()> {
        match self.bookmarks.get_mut(&key) {
            Some(bookmark) => {
                bookmark.directory = path;
                bookmark.updated_at = ::now();
            }
            None => bail!(ErrorKind::BookmarkNotFound(key)),
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

    pub fn list(&self) -> Iter<String, Bookmark> {
        self.bookmarks.iter()
    }

    fn close(&self) -> Result<()> {
        let path = PathBuf::from(&self.location);

        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&path)
            .chain_err(|| ErrorKind::DBOpenError(path))?;

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

#[cfg(test)]
mod tests {
    use super::{Bookmark};
    use std::path::PathBuf;

    #[test]
    fn bookmark_values() {
        let name = String::from("hello");
        let path = PathBuf::from("/tmp/hello");
        let bookmark = Bookmark::new(name, path);

        assert_eq!(bookmark.name, String::from("hello"));
        assert_eq!(bookmark.directory, PathBuf::from("/tmp/hello"));
        assert!(bookmark.last_access.is_none());
        assert_eq!(bookmark.count, 0);
    }
}
