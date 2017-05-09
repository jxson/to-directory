extern crate to;
#[macro_use]
extern crate slog;
#[macro_use]
extern crate error_chain;

use std::env;
use to::{cli, dir, logger};
use to::cli::Action;
use to::database::Database;
use to::errors::*;

fn main() {
    // change the error output and logging based on the flags.
    if let Err(ref e) = run() {
        use std::io::Write;
        let stderr = &mut ::std::io::stderr();
        let stderr_errmsg = "Error writing to stderr";

        writeln!(stderr, "error: {}", e).expect(stderr_errmsg);

        for e in e.iter().skip(1) {
            writeln!(stderr, "caused by: {}", e).expect(stderr_errmsg);
        }

        // The backtrace is not always generated. Try to run this example
        // with `RUST_BACKTRACE=1`.
        if let Some(backtrace) = e.backtrace() {
            writeln!(stderr, "backtrace: {:?}", backtrace).expect(stderr_errmsg);
        }

        ::std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let options = cli::parse();
    let log = logger::root(options.verbose);

    debug!(log, "logger initialized");

    info!(log, "parsed CLI options";
        "action" => format!("{:?}", options.action),
        "initialize" => options.initialize,
        "name" => format!("{:?}", options.name),
        "verbose" => options.verbose,
    );

    // to-directory --init # echo the shell script for the `to` function.
    if options.initialize {
        print!("{}", include_str!("to.sh"));
        return Ok(());
    }

    let config = try!(options.config());

    if !config.exists() {
        println!("does not exist {:?}", config);
        try!(dir::mkdirp(&config));
    }

    let store = try!(Database::open(config));
    info!(log, "database opened: {:?}", store.location);

    match options.action {
        Action::Info => info(&store, options),
        Action::Save => save(store, options),
        Action::Delete => delete(store, options),
        Action::List => list(&store),
        Action::Pathname => pathname(&store, options),
    }
}

fn info(store: &Database, options: cli::Options) -> Result<()> {
    let name = match options.name {
        Some(value) => value,
        None => bail!(ErrorKind::InfoFlagRequiresName),
    };

    let bookmark = try!(store.get(name));
    println!("bookmark: {:?}", bookmark);
    Ok(())
}

fn save(mut store: Database, options: cli::Options) -> Result<()> {
    let path = match options.path {
        Some(value) => try!(dir::resolve(value)),
        None => try!(env::current_dir()),
    };

    let basename = try!(dir::basename(&path));
    let name = match options.name {
        Some(value) => value,
        None => basename,
    };

    try!(store.put(name, path));

    Ok(())
}

fn delete(mut store: Database, options: cli::Options) -> Result<()> {
    let name = match options.name {
        Some(value) => value,
        None => bail!(ErrorKind::DeleteFlagRequiresName),
    };

    try!(store.delete(name));

    Ok(())
}

fn list(store: &Database) -> Result<()> {
    for (key, bookmark) in store.list() {
        println!("list: {}: {:?}", key, bookmark);
    }

    Ok(())
}

fn pathname(store: &Database, options: cli::Options) -> Result<()> {
    let name = match options.name {
        Some(value) => value,
        None => bail!(ErrorKind::ToRequiresName),
    };

    let bookmark = try!(store.get(name));
    let value = bookmark.directory.to_string_lossy();
    println!("{}", value);
    Ok(())
}
