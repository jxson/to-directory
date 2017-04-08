extern crate to;
#[macro_use]
extern crate slog;
extern crate error_chain;

use to::{cli, dir, logger};
use to::cli::Action;
use to::database::{ Database };
use to::errors::*;

fn main() {
    // change the error output and logging based on the flags.
    if let Err(ref e) = run() {
        use std::io::Write;
        let stderr = &mut ::std::io::stderr();
        let errmsg = "Error writing to stderr";

        writeln!(stderr, "error: {}", e).expect(errmsg);

        for e in e.iter().skip(1) {
            writeln!(stderr, "caused by: {}", e).expect(errmsg);
        }

        // The backtrace is not always generated. Try to run this example
        // with `RUST_BACKTRACE=1`.
        if let Some(backtrace) = e.backtrace() {
            writeln!(stderr, "backtrace: {:?}", backtrace).expect(errmsg);
        }

        ::std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let log = logger::root();
    debug!(log, "logger initialized");

    let options = cli::run();

    // TODO(jasoncampbell): change log level and frormat based on CLI flags.
    info!(log, "parsed CLI options";
        "action" => format!("{:?}", options.action),
        "initialize" => options.initialize,
        "name" => format!("{:?}", options.name),
        "verbose" => options.verbose,
    );

    // to-directory --init # echo the shell script for the `to` function.
    if options.initialize == true {
        print!("{}", include_str!("to.sh"));
        return Ok(());
    }

    let path = try!(dir::resolve(options.pathname));
    let basename = try!(dir::basename(&path));
    let name = match options.name {
        Some(value) => value,
        None => basename,
    };

    let config = try!(dir::config(options.config));
    info!(log, "config initialized"; "config" => config.to_str());

    let mut store = try!(Database::open(config));
    info!(log, "database opened");

    let result = match options.action {
        Action::Get => show(store, name),
        Action::Put => store.put(name, path),
        Action::Delete => store.delete(name),
        Action::List => list(store),
        Action::Pathname => pathname(store, name),
    };

    result
}

fn show(store: Database, name: String) -> Result<()> {
    let bookmark = try!(store.get(name));
    println!("bookmark: {:?}", bookmark);
    Ok(())
}

fn list(store: Database) -> Result<()> {
    for (key, bookmark) in store.list() {
        println!("list: {}: {:?}", key, bookmark);
    }

    return Ok(());
}

fn pathname(store: Database, name: String) -> Result<()> {
    let bookmark = try!(store.get(name));
    let value = bookmark.directory.to_string_lossy();
    println!("{}", value);
    Ok(())
}
