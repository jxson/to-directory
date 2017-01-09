use clap;

pub fn run() -> Options {
    let matches = CLI::matches();
    Options::new(matches)
}

pub fn _run(args: Vec<&str>) -> Options {
    let matches = CLI::matches_from(args);
    Options::new(matches)
}

#[derive(Debug)]
pub struct Options {
    pub verbose: bool,
}

impl Options {
    fn new(matches: clap::ArgMatches) -> Options {
        let verbose = matches.is_present("verbose");

        Options { verbose: verbose }
    }
}

struct CLI<'a> {
    app: clap::App<'a, 'a>,
}

impl<'a> CLI<'a> {
    fn new() -> CLI<'a> {
        let app = clap::App::new("to")
            .version(crate_version!())
            .author("Jason Campbell <jason@artifact.sh>")
            .about("Bookmark directories")
            .arg(clap::Arg::with_name("verbose")
                .short("v")
                .long("input")
                .help("Verbose log output")
                .takes_value(false));

        CLI { app: app }
    }

    fn matches() -> clap::ArgMatches<'a> {
        let cli = CLI::new();
        cli.app.get_matches()
    }

    fn matches_from(args: Vec<&str>) -> clap::ArgMatches {
        let cli = CLI::new();
        cli.app.get_matches_from(args)
    }
}
