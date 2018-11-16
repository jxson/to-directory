use bincode::{deserialize_from, serialize_into, Infinite};
use errors::{Error, ErrorKind, Result};
use failure::ResultExt;
use std::collections::btree_map::Iter;
use std::collections::BTreeMap;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;

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

        debug!("opening DB ");

        let bookmarks = File::open(&path)
            .map_err(Error::io)
            .and_then(hydrate)
            .with_context(|_| ErrorKind::Path(path.clone()))?;

        Ok(Database::new(path, bookmarks))
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

    // TODO(): fold get and get_path into one method.
    pub fn get_path(&mut self, key: String) -> Result<PathBuf> {
        let path: PathBuf;
        match self.bookmarks.get_mut(&key) {
            Some(bookmark) => {
                bookmark.last_access = Some(::now());
                path = bookmark.directory.clone();
            }
            None => return Err(Error::not_found(key)),
        };

        self.close()?;
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
        self.bookmarks
            .remove(&key)
            .ok_or_else(|| Error::not_found(key))
            .and_then(|_| self.close())
    }

    fn update(&mut self, key: String, path: PathBuf) -> Result<()> {
        match self.bookmarks.get_mut(&key) {
            Some(bookmark) => {
                bookmark.directory = path;
                bookmark.updated_at = ::now();
            }
            None => return Err(Error::not_found(key)),
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
        debug!("closing DB");

        let path = PathBuf::from(&self.location);

        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&path)
            .map_err(Error::io)
            .with_context(|_| ErrorKind::Path(path))?;

        dehydrate(file, &self.bookmarks)
    }
}

fn hydrate(file: File) -> Result<Bookmarks> {
    let mut reader = BufReader::new(file);
    let bookmarks: Bookmarks = deserialize_from(&mut reader, Infinite).map_err(Error::bincode)?;
    Ok(bookmarks)
}

fn dehydrate(file: File, bookmarks: &Bookmarks) -> Result<()> {
    let mut writer = BufWriter::new(file);

    serialize_into(&mut writer, &bookmarks, Infinite).map_err(Error::bincode)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::Bookmark;
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
