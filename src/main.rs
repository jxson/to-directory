extern crate to;
#[macro_use]
extern crate slog;

use to::logger;
use to::cli;
use to::cli::Action;
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

    // TODO(jasoncampbell): change log level and frormat based on CLI flags.
    info!(log, "parsed CLI options";
        "action" => format!("{:?}", options.action),
        "initialize" => options.initialize,
        "name" => format!("{:?}", options.name),
        "verbose" => options.verbose,
    );

    // to-directory --init # echo the shell script for the `to` function.
    if options.initialize == true {
        print!("{}", include_str!("to.sh"));
        return Ok(());
    }

    let result = match options.action {
        Action::Get => show(options.name),
        _ => panic!("Not implemented"),
    };

    Ok(())
}

fn show(name: Option<String>) -> Result<()> {
    Ok(())
}
