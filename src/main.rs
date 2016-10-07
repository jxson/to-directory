#[macro_use]
extern crate clap;

#[macro_use]
extern crate time;

#[macro_use]
extern crate rustc_serialize;

extern crate bincode;
#[macro_use]
extern crate log;
extern crate env_logger;

mod cli;
mod dir;
mod database;
mod error;
#[macro_use]
mod logger;

use cli::{Action};
use database::{Database};
use error::{ToResult};

fn main() {
    let request = match cli::Request::get() {
        Ok(value) => value,
        Err(err) => exit!("Failed to parse CLI args.\n  {:?}", err),
    };

    if let Err(err) = logger::init(request.verbose) {
        exit!("Error initializing logger.\n {:?}", err);
    }

    let db_path = match dir::db() {
        Ok(value) => value,
        Err(err) => exit!("Error setting up DB path.\n {:?}", err),
    };

    let mut store = match Database::open(db_path) {
        Ok(db) => db,
        Err(err) => exit!("Failed to open DB.\n {:?}", err),
    };

    let result = match request.action {
        Action::Initialize => init(),
        Action::Get => show(store, request.name),
        Action::Put => store.put(request.name, request.directory),
        Action::Delete => store.delete(request.name),
        Action::List => list(store),
        Action::ChangeDirectory => cd(store, request),
        _ => panic!("'{:?}' NOT IMPLEMENTED!", request.action),
    };

    match result {
        Ok(_) => {},
        Err(err) => panic!(err),
    }
}

fn cd(store: Database, request: cli::Request) -> ToResult<()> {
    if let Some(bookmark) = store.get(&request.name) {
        println!("result {}", bookmark.directory.to_string_lossy());
    } else {
        panic!("NOT FOUND");
    }

    return Ok(());
}

fn show(store: Database, key: String) -> ToResult<()> {
    if let Some(bookmark) = store.get(&key) {
        println!("info: {:?}", bookmark);
    } else {
        panic!("NOT FOUND");
    }

    return Ok(());
}

fn list(store: Database) -> ToResult<()> {
    for (key, bookmark) in store.all() {
        println!("list: {}: {:?}", key, bookmark);
    }

    return Ok(());
}

// to init: prints instructions
// to init -: echos shell
fn init() -> ToResult<()> {
    let script = include_str!("to.sh");

    print!("{}", script);

    return Ok(());
}
