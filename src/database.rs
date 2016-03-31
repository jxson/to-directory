use std::path::{ PathBuf };
use types::{ ToResult, ToError, Bookmark, Bookmarks };
use dir::{ mkdirp };
use std::fs::{ File };
use std::io::{ BufReader, BufWriter, ErrorKind };
use bincode::SizeLimit::Infinite;
use bincode::rustc_serialize::{ encode_into, decode_from };
use std::string::String;

// use std::path::{PathBuf, Components};

#[derive(Debug)]
pub struct Database {
    location: PathBuf,
    // bookmarks: Cell<Bookmarks>,
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
    fn new(location: PathBuf) -> Database {
        Database {
            location: location,
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
            },
            Err(ref err) if err.kind() == ErrorKind::NotFound => {
                Bookmarks::new()
            },
            Err(err) => return Err(ToError::Io(err)),
        };

        let db = Database::new(location);
        return Ok(db);
    }

    // // TODO: open a new db without wrtiting to it first.
    // fn bootstrap(location: &PathBuf) -> ToResult<File> {
    //     println!("bootstrapping {:?}", location);
    //     let file = match File::create(&location) {
    //         Ok(value) => value,
    //         Err(err) => return Err(ToError::Io(err)),
    //     };
    //
    //     let bookmarks = Bookmarks::new();
    //     let mut writer = BufWriter::new(file);
    //     match encode_into(&bookmarks, &mut writer, SizeLimit::Infinite) {
    //         Ok(_) => println!("successful encode"),
    //         Err(err) => panic!("ERROR ECODING: {:?}", err),
    //     }
    //
    //     match File::open(location) {
    //         Ok(value) => return Ok(value),
    //         Err(err) => return Err(ToError::Io(err)),
    //     };
    // }

    pub fn put(&self, key: String, value: &PathBuf) -> ToResult<()> {
        println!("db.put({:?}, {:?})", key, value);
        // Check that the db has been opened.

        return Ok(());
    }


    //         let mut collection = match self.all() {
    //             Ok(value) => value,
    //             Err(err) => panic!(err),
    //         };
    //
    //         let key = name.clone();
    //         let value = Bookmark::new(name, directory);
    //         collection.insert(key, value);
    //
    //         println!("collection {:?}", collection);
    //
    //         if let Err(err) = self.write(collection) {
    //             print!("Error on put");
    //         }
    //
    //         return Ok(());
    //     }

}


//
//
//     fn read(&self) -> ToResult<Bookmarks> {
//         let file = match open(&self.location) {
//             Ok(value) => value,
//             Err(err) => return Err(err),
//         };
//
//         let mut reader = BufReader::new(file);
//         let bookmarks: Bookmarks = match decode_from(&mut reader, SizeLimit::Infinite) {
//             Ok(value) => value,
//             Err(err) => panic!("DECODING ERROR"),
//         };
//
//         println!("existing {:?}", bookmarks);
//
//         return Ok(bookmarks);
//     }
//
//     fn write(&self, bookmarks: Bookmarks) -> ToResult<()> {
//         println!("Writing db to {:?}", self.location);
//         println!(" * bookmarks {:?}", bookmarks);
//
//         let file = match open(&self.location) {
//             Ok(value) => value,
//             Err(err) => return Err(err),
//         };
//
//         println!("file {:?}", file);
//
//         let bytes: Vec<u8> = match encode(&bookmarks, SizeLimit::Infinite) {
//             Ok(value) => value,
//             Err(err) => panic!(err),
//         };
//
//         println!("encoded bytes {:?}", bytes);
//
//         return Ok(());
//     }
// }
//
//
