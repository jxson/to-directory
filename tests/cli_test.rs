extern crate to;

use to::cli;

#[test]
fn cli_run() {
    let options = match cli::run() {
        Ok(options) => options,
        Err(err) => panic!(err),
    };

    assert_eq!(options.verbose, false);
}
