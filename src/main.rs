#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate bincode;
#[macro_use]
extern crate rustc_serialize;

mod cli;
mod dir;
mod database;
mod error;
#[macro_use]
mod logger;

use database::{Database};
use cli::{Action};
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

    println!("store {:?}", store);
    let result = match request.action {
        Action::Put => store.put(request.name, request.directory),
        Action::Get => show(store, request.name),
        _ => panic!("NOT IMPLEMENTED!"),
    };

    match result {
        Ok(value) => println!("success! {:?}", value),
        Err(err) => panic!(err),
    }
}

fn show(mut store: Database, key: String) -> ToResult<()> {
    let bookmark = try!(store.get(key));
    println!("info: {:?}", bookmark);
    return Ok(());
}
