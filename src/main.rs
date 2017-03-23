
extern crate to;
#[macro_use]extern crate slog;
extern crate slog_bunyan;
extern crate slog_stream;
#[macro_use] extern crate serde_json;

use slog::Drain;
use std::sync::Mutex;
use to::cli;
use to::errors::*;

fn main() {
    // change the error output and logging based on the flags.
    if let Err(ref e) = run() {
        use ::std::io::Write;
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
    // Initialize logger https://github.com/slog-rs/slog
    let log = slog::Logger::root(
            Mutex::new(
                slog_bunyan::default(
                    std::io::stderr()
                )
            ).fuse(),
            o!("version" => env!("CARGO_PKG_VERSION"))
    );

    // let log =
    info!(log, "foo"; "stage" => "end");

    let options = cli::run();
    let log = log.new(o!("module" => "cli"));

    // info!(log, "hello"; "options" => options);
    info!(log, "hello"; "options" => options);

    // serde_json::to_value

    // Get json rendering for logger.
    // https://github.com/loggerhead/shadowsocks-rust/blob/master/src/my_logger.rs

    // info!(self.log, "wrote recording"; "uuid" => format_args!("{}", r.sample_file_uuid.hyphenated()));
    // println!("{:?}", options);
    Ok(())
}
