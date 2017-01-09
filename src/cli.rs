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
        Options { verbose: matches.is_present("verbose") }
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
                .long("verbose")
                .short("v")
                .help("Verbose log output")
                .takes_value(false));

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
