use cli;
use slog_async;
use slog_term;
use slog;
use slog::{Drain, Level};
use std;

pub use slog::Logger;

pub fn root(options: &cli::Options) -> slog::Logger {
    let decorator = slog_term::PlainDecorator::new(std::io::stdout());
    let drain = slog_term::CompactFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    let level = if options.verbose {
        Level::Info
    } else {
        Level::Error
    };

    let drain = drain.filter_level(level).fuse();

    let log = slog::Logger::root(
        drain,
        o!("name" => "to", "version" => env!("CARGO_PKG_VERSION")),
    );

    info!(log, "parsed CLI options";
        "action" => format!("{:?}", options.action),
        "initialize" => options.initialize,
        "name" => format!("{:?}", options.name),
        "verbose" => options.verbose,
    );

    log
}
