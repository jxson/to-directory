#[macro_use]
extern crate clap;
#[macro_use]
extern crate time;
extern crate bincode;
#[macro_use]
extern crate rustc_serialize;

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
        Err(err) => panic!(err),
    };

    logger::init(request.verbose);
    log!("Logger works!");
    log!("request: {:?}", request);

    let db = match dir::db() {
        Ok(value) => value,
        Err(err) => panic!(err),
    };

    log!("db: {:?}", db);

    let mut store = match Database::open(db) {
        Ok(db) => db,
        Err(e) => panic!("failed to open database: {:?}", e),
    };

    log!("store: {:?}", store);

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
        print!("{}", bookmark.directory.to_string_lossy());
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
