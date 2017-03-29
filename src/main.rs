
extern crate to;
#[macro_use]
extern crate slog;
extern crate slog_json;
extern crate slog_stream;
#[macro_use]
extern crate serde_json;
extern crate chrono;

use to::logger;
use to::cli;
use to::errors::*;

fn main() {
    // change the error output and logging based on the flags.
    if let Err(ref e) = run() {
        use std::io::Write;
        let stderr = &mut ::std::io::stderr();
        let errmsg = "Error writing to stderr";

        writeln!(stderr, "error: {}", e).expect(errmsg);

        for e in e.iter().skip(1) {
            writeln!(stderr, "caused by: {}", e).expect(errmsg);
        }

        // The backtrace is not always generated. Try to run this example
        // with `RUST_BACKTRACE=1`.
        if let Some(backtrace) = e.backtrace() {
            writeln!(stderr, "backtrace: {:?}", backtrace).expect(errmsg);
        }

        ::std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let log = logger::root();
    debug!(log, "logger initialized");

    let options = cli::run();

    info!(log, "parsed CLI options";
        "action" => format!("{:?}", options.action),
        "initialize" => options.initialize,
        "name" => options.name,
        "verbose" => options.verbose,
    );

    Ok(())
}
