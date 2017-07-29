extern crate to;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate prettytable;
#[macro_use]
extern crate slog;

use std::path::PathBuf;
use prettytable::Table;
use to::{cli, dir, logger};
use to::cli::Action;
use to::database::Database;
use to::errors::*;
use std::io::Write;

fn main() {
    let matches = cli::app().get_matches();
    let mut out = std::io::stdout();

    if let Err(ref e) = run(matches, &mut out) {
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

fn run<T: Write + ?Sized>(matches: cli::ArgMatches, out: &mut T) -> Result<()> {
    let options = try!(cli::Options::new(matches));
    let log = logger::root(&options);

    // --init # echo the shell script for the `to` function.
    if options.initialize {
        print!("{}", include_str!("to.sh"));
        return Ok(());
    }

    let config = PathBuf::from(&options.config);

    if !config.exists() {
        try!(dir::mkdirp(&config));
    }

    let mut store = try!(Database::open(config));
    info!(log, "database opened: {:?}", store.location);

    match options.action {
        Action::Info => info(&store, &options),
        Action::Save => store.put(options.name, options.path),
        Action::Delete => store.delete(options.name),
        Action::List => list(&store, out),
        Action::Pathname => pathname(&store, options),
    }
}

fn info(store: &Database, options: &cli::Options) -> Result<()> {
    match store.get(&options.name) {
        Some(bookmark) => println!("bookmark: {:?}", bookmark),
        None => println!("Not found"),
    }

    Ok(())
}

fn list<T: Write + ?Sized>(store: &Database, out: &mut T) -> Result<()> {
    let mut table = Table::new();
    table.add_row(row![ b => "Name", "Path", "Count"]);

    for (name, bookmark) in store.list() {
        let path = bookmark.directory.to_string_lossy();
        table.add_row(row![name, path, bookmark.count]);
    }

    try!(table.print(out));

    Ok(())
}

fn pathname(store: &Database, options: cli::Options) -> Result<()> {
    let value = match store.get(&options.name) {
        Some(bookmark) => bookmark.directory.to_string_lossy(),
        None => bail!(ErrorKind::BookmarkNotFound(options.name)),
    };

    println!("{}", value);

    Ok(())
}

#[cfg(test)]
mod test {
    extern crate tempdir;

    use super::*;
    use self::tempdir::TempDir;
    use std::io::{self, Write};

    struct TestWriter {}

    impl Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            Ok(buf.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    fn test_run(matches: cli::ArgMatches) -> Result<()> {
        let mut out = TestWriter {};
        run(matches, &mut out)
    }

    fn get_matches(values: Vec<&str>) -> cli::ArgMatches {
        let path = TempDir::new("test-config").map(|temp| temp.into_path());
        let config = path.as_ref().map(|path| path.to_str().unwrap()).unwrap();

        let mut args = vec!["to", "--config", config];
        args.extend(values);

        cli::app().get_matches_from(args)
    }

    #[test]
    fn run_is_ok() {
        let matches = get_matches(vec!["--info"]);
        let result = test_run(matches);
        assert!(result.is_ok());
    }

    #[test]
    fn run_with_init_flag() {
        let matches = get_matches(vec!["--init"]);
        let result = test_run(matches);
        assert!(result.is_ok());
    }

    #[test]
    fn run_with_non_existing_config() {
        let config = TempDir::new("existing-dir")
            .map(|dir| dir.into_path().join("non-existing"))
            .unwrap();
        let config_value = config.to_str().unwrap();
        let matches = cli::app().get_matches_from(vec!["to", "--config", config_value, "--info"]);

        assert_eq!(config.exists(), false);
        assert!(test_run(matches).is_ok());
        assert!(config.exists());
    }

    #[test]
    fn run_with_save_flag() {
        let matches = get_matches(vec!["--save"]);
        let result = test_run(matches);
        assert!(result.is_ok());

        let matches = get_matches(vec!["--save", "foo"]);
        let result = test_run(matches);
        assert!(result.is_ok());
    }

    #[test]
    fn run_with_delete_flag() {
        let matches = get_matches(vec!["--delete", "foo"]);
        let key = String::from("foo");
        let err = test_run(matches).err().unwrap();
        assert_eq!(
            format!("{}", ErrorKind::BookmarkNotFound(key)),
            format!("{}", err)
        );
    }

    #[test]
    fn run_with_list_flag() {
        let matches = get_matches(vec!["--list"]);
        let result = test_run(matches);
        assert!(result.is_ok());
    }

    #[test]
    fn run_with_name() {
        let matches = get_matches(vec!["foo"]);
        let key = String::from("foo");
        let err = test_run(matches).err().unwrap();
        assert_eq!(
            format!("{}", ErrorKind::BookmarkNotFound(key)),
            format!("{}", err)
        );
    }
}
