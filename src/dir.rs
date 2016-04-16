use std::path::{Path, PathBuf};
use std::env;
use types::ToResult;

pub fn config() -> ToResult<PathBuf> {
    let mut directory = match env::home_dir() {
        Some(value) => value,
        None => panic!("TODO: Custom error - unable to locate home directory."),
    };

    directory.push(".to");

    return Ok(directory);
}

pub fn mkdirp(directory: &Path) -> ToResult<()> {
    return Ok(());
}

// fn mkdirp(directory: &path::PathBuf) -> ToResult<()> {
//     let mut directory = path::PathBuf::from(directory);
//             directory.push(".to");
//
//     match fs::create_dir(&directory) {
//         Ok(_) => return Ok(()),
//         Err(ref err) if exists(err) => return Ok(()),
//         Err(err) => return Err(ToError::Io(err)),
//     }
//
//     // return Ok(dir);
// }

// fn exists(err: &io::Error) -> bool {
//     return err.kind() == io::ErrorKind::AlreadyExists;
// }

// fn bootstrap(directory: &path::PathBuf) -> ToResult<path::PathBuf> {
//     // Create a directory in ~/.to if it doesn't already exist.
//     if let Err(err) = mkdirp(directory) {
//         return Err(err)
//     }
//
//     // match mkdir(home) {
//     //     Ok(directory) => return Ok(Store { path: Some(directory) }),
//     //     Err(err) => return Err(err),
//     // };
//
//     // Create a databse file if it doesn't exist already.
//     let mut buf = path::PathBuf::from(directory);
//             buf.set_file_name("db.bin");
//
//     match fs::File::create(&buf) {
//         Ok(_) => {},
//         Err(ref err) if exists(err) => {},
//         Err(err) => return Err(ToError::Io(err)),
//     }
//
//     return Ok(buf);
// }
//
// fn mkdirp(directory: &path::PathBuf) -> ToResult<()> {
//     let mut directory = path::PathBuf::from(directory);
//             directory.push(".to");
//
//     match fs::create_dir(&directory) {
//         Ok(_) => return Ok(()),
//         Err(ref err) if exists(err) => return Ok(()),
//         Err(err) => return Err(ToError::Io(err)),
//     }
//
//     // return Ok(dir);
// }
//
// fn exists(err: &io::Error) -> bool {
//     return err.kind() == io::ErrorKind::AlreadyExists;
// }
//
