extern crate to;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate prettytable;
#[macro_use]
extern crate slog;

use std::path::PathBuf;
use prettytable::Table;
use std::env;
use to::{cli, dir, logger};
use to::cli::Action;
use to::database::Database;
use to::errors::*;

fn main() {
    let matches = cli::app().get_matches();
    let (log, options) = setup(matches);

    // change the error output and logging based on the flags.
    if let Err(ref e) = run(&log, options) {
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

/// Reduces boilerplate, returns a working logger and parsed CLI options.
fn setup(matches: cli::ArgMatches) -> (logger::Logger, cli::Options) {
    let options = cli::Options::new(matches);
    let log = logger::root(&options);

    (log, options)
}

fn run(log: &slog::Logger, options: cli::Options) -> Result<()> {
    // --init # echo the shell script for the `to` function.
    if options.initialize {
        print!("{}", include_str!("to.sh"));
        return Ok(());
    }

    let config = match options.config {
        // NOTE(jxson): I am sure there is a better way to do this but I am a n00b.
        Some(ref value) => PathBuf::from(value),
        None => bail!(ErrorKind::ConfigError),
    };

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
    let mut table = Table::new();
    table.add_row(row![ b => "Name", "Path", "Count"]);

    for (name, bookmark) in store.list() {
        let path = bookmark.directory.to_string_lossy();
        table.add_row(row![name, path, bookmark.count]);
    }

    table.printstd();

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

#[cfg(test)]
mod test {
    extern crate tempdir;

    use super::*;
    use self::tempdir::TempDir;

    #[test]
    fn setup_is_ok() {
        let tmp = TempDir::new("blah").unwrap();
        let config = tmp.path().to_str().unwrap();
        let matches = cli::app().get_matches_from(vec!["to", "--config", config]);
        let (_, options) = setup(matches);

        assert_eq!(options.verbose, false);
        assert_eq!(options.initialize, false);
        assert_eq!(options.name, None);
        assert_eq!(options.action, Action::Pathname);
    }

    #[test]
    fn run_with_init_flag() {
        let matches = cli::app().get_matches_from(vec!["to", "--init"]);
        let (log, options) = setup(matches);
        let result = run(&log, options);
        assert!(result.is_ok());
    }

    #[test]
    fn run_with_bad_config() {
        let matches = cli::app().get_matches_from(vec!["to"]);
        let (log, mut options) = setup(matches);
        options.config = None;
        let err = run(&log, options).err().unwrap();
        assert_eq!(format!("{}", err), format!("{}", ErrorKind::ConfigError {}));
    }
}
