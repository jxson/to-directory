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
        return Options {
            verbose: false,
        };
    }
}

struct CLI<'a> {
    app: clap::App<'a, 'a>,
}

impl<'a,> CLI<'a> {
    fn new() -> CLI<'a> {
        let app = clap::App::new("to")
            .version(crate_version!())
            .author("Jason Campbell <jason@artifact.sh>")
            .about("Bookmark directories")
            .arg(clap::Arg::with_name("verbose")
                .help("Verbose log output")
                .short("v"));

        CLI { app: app }
    }

    fn matches() -> clap::ArgMatches<'a> {
        let cli = CLI::new();
        return cli.app.get_matches();
    }

    fn matches_from(args: Vec<&str>) -> clap::ArgMatches {
        let cli = CLI::new();
        return cli.app.get_matches_from(args);
    }
}
