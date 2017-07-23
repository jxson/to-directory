extern crate to;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate prettytable;
#[macro_use]
extern crate slog;

use std::env::current_dir;
use std::path::PathBuf;
use prettytable::Table;
use to::{cli, dir, logger};
use to::cli::Action;
use to::database::Bookmark;
use to::database::Database;
use to::errors::*;

fn main() {
    let matches = cli::app().get_matches();

    // change the error output and logging based on the flags.
    if let Err(ref e) = run(matches) {
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

fn run(matches: cli::ArgMatches) -> Result<()> {
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
        Action::SaveRelative => save_relative(&mut store, options.name, options.path),
        Action::Delete => store.delete(options.name),
        Action::List => list(&store),
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

fn save_relative(store: &mut Database, key: String, value: PathBuf) -> Result<()> {
    if let Some(stripped_path) = store.strip_longest_path_prefix_match(&value) {
        store.put(key, stripped_path.to_path_buf())
    } else {
        println!("New bookmark path '{:?}' doesn't match any existing absolute bookmarks!",
                 value);
        Ok(())
    }
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

fn maybe_transform_relative_bookmark(store: &Database, bookmark: &Bookmark,
                                     effective_dir: PathBuf) -> PathBuf {
    if bookmark.directory.is_relative() {
        if let Some(absolute_bookmark) = store.find_longest_path_prefix_match(&effective_dir) {
            absolute_bookmark.directory.join(bookmark.directory.as_path().clone())
        } else {
            bookmark.directory.clone()
        }
    } else {
        bookmark.directory.clone()
    }
}

fn pathname(store: &Database, options: cli::Options) -> Result<()> {
    let effective_dir = match current_dir() {
        Ok(dir) => dir,
        Err(_) => bail!(ErrorKind::CurrentDirectoryError(PathBuf::from("."))),
    };
    let value = match store.get(&options.name) {
        Some(bookmark) =>
            maybe_transform_relative_bookmark(store, bookmark, effective_dir),
        None => bail!(ErrorKind::BookmarkNotFound(options.name)),
    };

    println!("{}", value.to_string_lossy());

    Ok(())
}

#[cfg(test)]
mod test {
    extern crate tempdir;

    use super::*;
    use self::tempdir::TempDir;

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
        let result = run(matches);
        assert!(result.is_ok());
    }

    #[test]
    fn run_with_init_flag() {
        let matches = get_matches(vec!["--init"]);
        let result = run(matches);
        assert!(result.is_ok());
    }

    #[test]
    fn run_with_non_existing_config() {
        let config = TempDir::new("existing-dir")
            .map(|dir| dir.into_path().join("non-existing"))
            .unwrap();
        let config_value = config.to_str().unwrap();
        let matches = cli::app().get_matches_from(vec!["to", "--config", config_value, "--info"]);

        assert!(!config.exists());
        assert!(run(matches).is_ok());
        assert!(config.exists());
    }
}
