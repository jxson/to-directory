use clap;

pub fn run() -> Options {
    let matches = CLI::matches();
    Options::new(matches)
}

pub fn from(args: Vec<&str>) -> Options {
    let matches = CLI::matches_from(args);
    Options::new(matches)
}

#[derive(Debug, PartialEq)]
pub enum Action {
    Delete,
    Get,
    List,
    Put,
    Pathname,
}

#[derive(Debug, PartialEq)]
pub struct Options {
    pub verbose: bool,
    pub initialize: bool,
    pub action: Action,
    pub name: Option<String>,
    pub pathname: String,
    pub config: Option<String>,
}

impl Options {
    fn new(matches: clap::ArgMatches) -> Options {
        let (delete, get, list, put) = (matches.is_present("delete"),
                                        matches.is_present("get"),
                                        matches.is_present("list"),
                                        matches.is_present("put"));

        let action = match (delete, get, list, put) {
            (true, _, _, _) => Action::Delete,
            (_, true, _, _) => Action::Get,
            (_, _, true, _) => Action::List,
            (_, _, _, true) => Action::Put,
            _ => Action::Pathname,
        };

        let config = matches.value_of("config").map(|value| String::from(value));

        let name = matches.value_of("NAME").map(|value| String::from(value));

        let pathname = String::from(matches.value_of("DIRECTORY").unwrap_or(""));

        Options {
            action: action,
            config: config,
            pathname: pathname,
            initialize: matches.is_present("initialize"),
            name: name,
            verbose: matches.is_present("verbose"),
        }
    }
}

struct CLI<'a> {
    app: clap::App<'a, 'a>,
}

impl<'a> CLI<'a> {
    fn new() -> CLI<'a> {
        let app = clap::App::new("to")
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
                .takes_value(false))

            // Positional arguments.
            .arg(clap::Arg::with_name("NAME")
                .help("Name of the bookamrk")
                .index(1))
            .arg(clap::Arg::with_name("DIRECTORY")
                .help("Path of the bookamrk")
                .index(2))

            // Flags.
            .arg(clap::Arg::with_name("get")
                .long("info")
                .short("i")
                .help("Show bookmark information")
                .takes_value(false))
            .arg(clap::Arg::with_name("put")
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
                    "list",
                ]));

        CLI { app: app }
    }

    fn matches() -> clap::ArgMatches<'a> {
        let cli = CLI::new();
        cli.app.get_matches()
    }

    fn matches_from(mut args: Vec<&str>) -> clap::ArgMatches {
        let cli = CLI::new();
        args.insert(0, "to");
        cli.app.get_matches_from(args)
    }
}
