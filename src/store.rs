use std::path::{PathBuf, Components};
use types::{ToResult, ToError, Bookmark, Bookmarks};
use std::fs::{File};
use std::io::{BufReader, BufWriter, ErrorKind};
use dir::{mkdirp};
use bincode::SizeLimit;
use bincode::rustc_serialize::{encode, encode_into, decode_from};

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

        let mut reader = BufReader::new(file);
        let bookmarks: Bookmarks = match decode_from(&mut reader, SizeLimit::Infinite) {
            Ok(value) => value,
            Err(err) => panic!("DECODING ERROR"),
        };

        println!("existing {:?}", bookmarks);

        return Ok(bookmarks);
    }

    fn write(&self, bookmarks: Bookmarks) -> ToResult<()> {
        println!("Writing db to {:?}", self.location);
        println!(" * bookmarks {:?}", bookmarks);

        let file = match open(&self.location) {
            Ok(value) => value,
            Err(err) => return Err(err),
        };

        println!("file {:?}", file);

        let bytes: Vec<u8> = match encode(&bookmarks, SizeLimit::Infinite) {
            Ok(value) => value,
            Err(err) => panic!(err),
        };

        println!("encoded bytes {:?}", bytes);

        match file.write_all(&bytes) {
            Ok(value) => return Ok(value),
            Err(err) => panic!("Error writing encoded DB"),
        }

        return Ok(());
    }
}

fn open(location: &PathBuf) -> ToResult<File> {
    let directory = match location.parent() {
        Some(value) => value,
        None => panic!("location cannot be a directory."),
    };

    if let Err(err) = mkdirp(directory) {
        return Err(err);
    }

    let file = match File::open(location) {
        Ok(value) => value,
        Err(ref err) if err.kind() == ErrorKind::NotFound => {
            match bootstrap(location) {
                Ok(value) => return Ok(value),
                Err(err) => return Err(err),
            }
        },
        Err(err) => return Err(ToError::Io(err)),
    };

    return Ok(file);
}

// TODO: open a new db without wrtiting to it first.
fn bootstrap(location: &PathBuf) -> ToResult<File> {
    println!("bootstrapping {:?}", location);
    let file = match File::create(&location) {
        Ok(value) => value,
        Err(err) => return Err(ToError::Io(err)),
    };

    let bookmarks = Bookmarks::new();
    let mut writer = BufWriter::new(file);
    match encode_into(&bookmarks, &mut writer, SizeLimit::Infinite) {
        Ok(_) => println!("successful encode"),
        Err(err) => panic!("ERROR ECODING: {:?}", err),
    }

    match File::open(location) {
        Ok(value) => return Ok(value),
        Err(err) => return Err(ToError::Io(err)),
    };
}
