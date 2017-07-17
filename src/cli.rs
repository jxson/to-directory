use clap;
use std;
use std::env;
use std::path::PathBuf;
use errors::*;
use dir;

pub use clap::ArgMatches;

pub fn app<'a, 'b>() -> clap::App<'a, 'b> {
    clap::App::new("to")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Bookmark directories")

        // User friendly info! output.
        .arg(clap::Arg::with_name("verbose")
            .long("verbose")
            .short("v")
            .help("Verbose log output")
            .takes_value(false))

        .arg(clap::Arg::with_name("config")
            .long("config")
            .short("c")
            .help("Config dir, defaults to ~/.to")
            .takes_value(true))

        // Positional arguments.
        .arg(clap::Arg::with_name("NAME")
            .help("Name of the bookamrk")
            .index(1))
        .arg(clap::Arg::with_name("DIRECTORY")
            .help("Path of the bookamrk")
            .index(2))

        // Flags.
        .arg(clap::Arg::with_name("info")
            .long("info")
            .short("i")
            .help("Show bookmark information")
            .takes_value(false))
        .arg(clap::Arg::with_name("save")
            .long("save")
            .short("s")
            .help("Save bookmark")
            .takes_value(false))
        .arg(clap::Arg::with_name("delete")
            .long("delete")
            .short("d")
            .help("Delete bookmark")
            .takes_value(false)
            .requires("NAME"))
        .arg(clap::Arg::with_name("list")
            .long("list")
            .short("l")
            .help("List all bookmarks")
            .takes_value(false))
        .arg(clap::Arg::with_name("initialize")
            .long("init")
            .help("Echo initialization script")
            .takes_value(false)
            .conflicts_with_all(&[
                "NAME",
                "DIRECTORY",
                "get",
                "put",
                "delete",
                "list"]))
}

#[derive(Debug, PartialEq)]
pub enum Action {
    Delete,
    Info,
    List,
    Save,
    Pathname,
}

pub struct Options {
    pub verbose: bool,
    pub initialize: bool,
    pub action: Action,
    pub name: String,
    pub path: PathBuf,
    pub config: PathBuf,
}

impl Options {
    /// Creates a new instance of Options from clap::ArgMatches.
    ///
    /// # Examples:
    ///
    /// ```
    /// use to::cli;
    ///
    /// let matches = cli::app().get_matches_from(vec![""]);
    /// let options = cli::Options::new(matches).unwrap();
    ///
    /// assert_eq!(options.verbose, false);
    /// assert_eq!(options.initialize, false);
    /// assert_eq!(options.action, cli::Action::Pathname);
    /// ```
    pub fn new(matches: clap::ArgMatches) -> Result<Options> {
        let (delete, info, list, save) = (
            matches.is_present("delete"),
            matches.is_present("info"),
            matches.is_present("list"),
            matches.is_present("save"),
        );

        let action = match (delete, info, list, save) {
            (true, _, _, _) => Action::Delete,
            (_, true, _, _) => Action::Info,
            (_, _, true, _) => Action::List,
            (_, _, _, true) => Action::Save,
            _ => Action::Pathname,
        };

        let directory = matches.value_of("DIRECTORY").map(PathBuf::from);
        let path = match directory {
            Some(value) => try!(dir::resolve(value)),
            None => try!(env::current_dir()),
        };

        let name = matches
            .value_of("NAME")
            .map(normalize)
            .unwrap_or(try!(dir::basename(&path)));

        Ok(Options {
            action: action,
            config: try!(config(matches.value_of("config"))),
            path: path,
            initialize: matches.is_present("initialize"),
            name: name,
            verbose: matches.is_present("verbose"),
        })
    }
}

// Normalize CLI text input.
//
// Will convert to lowercase, remove whitespace, and trim trailing slashes sometimes added by tab
// completion.
fn normalize(string: &str) -> String {
    string
        .trim()
        .trim_right_matches(std::path::MAIN_SEPARATOR)
        .to_lowercase()
}

fn config(value: Option<&str>) -> Result<PathBuf> {
    value
        .map(PathBuf::from)
        .or_else(|| {
            env::home_dir().map(|mut home| {
                home.push(".to");
                home
            })
        })
        .ok_or_else(|| ErrorKind::ConfigError.into())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn normalize_name_input() {
        assert_eq!(normalize("trim-trailing/"), "trim-trailing");
        assert_eq!(normalize("LOWERCASE"), "lowercase");
        assert_eq!(normalize("  spaces "), "spaces");
    }
}
