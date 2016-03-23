use std::path::{PathBuf, Components};
use types::{ToResult, ToError, Bookmark, Bookmarks};
use std::fs::{File};
use std::io::{BufReader, BufWriter, ErrorKind};
use dir::{mkdirp};
use bincode::SizeLimit;
use bincode::rustc_serialize::{encode_into};

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
        //
        //
        // println!("reader {:?}", reader);
        // println!("decoded {:?}", decoded);
        //
        let bookmarks = Bookmarks::new();

        return Ok(bookmarks);
    }

            //
            // let mut collection: BTreeMap<String, bookmark::Bookmark> = BTreeMap::new();
            // let key = bm.name.clone();
            // collection.insert(key, bm);
            //
            //
            // let encoded: Vec<u8> = encode(&collection, SizeLimit::Infinite).unwrap();
            //
            // println!("encoded {:?}", encoded);
            //
            // let decoded: BTreeMap<String, bookmark::Bookmark> = decode(&encoded[..]).unwrap();
            //
            // println!("decoded {:?}", decoded["foo"]);

    fn write(&self, bookmarks: Bookmarks) -> ToResult<()> {
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
            bootstrap(location);
        },
        Err(err) => return Err(ToError::Io(err)),
    };

    return Ok(file);
}

fn bootstrap(location: &PathBuf) -> ToResult<()> {
    let file = match File::create(&location) {
        Ok(value) => value,
        // Err(ref err) if err.kind() == ErrorKind::AlreadyExists => {},
        Err(err) => return Err(ToError::Io(err)),
    };

    let bookmarks = Bookmarks::new();
    let mut writer = BufWriter::new(file);
    match encode_into(&bookmarks, &mut writer, SizeLimit::Infinite) {
        Ok(_) => println!("successful encode"),
        Err(err) => panic!("ERROR ECODING: {:?}", err),
    }

}
